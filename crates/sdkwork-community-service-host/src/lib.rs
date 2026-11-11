use std::sync::Arc;

use sdkwork_community_database_host::bootstrap_community_database;
pub use sdkwork_community_service::MembershipPackagePublisher;
pub use sdkwork_community_service::OrderPaymentVerifier;
use sdkwork_community_service::{CommerceIntegration, CommerceTransportMode, CommunityService};
use sdkwork_community_storage_sqlx::CommunitySqlxStore;
use sdkwork_database_id::{
    IdGenerator, NodeAllocatorConfig, NodeLease, SnowflakeIdGenerator, SnowflakeNodeAllocator,
};
use sdkwork_database_sqlx::DatabasePool;

/// Logical service name used for database-backed snowflake node allocation.
pub const COMMUNITY_SNOWFLAKE_SERVICE_NAME: &str = "community-service";

/// Static snowflake node id fallback for dev/test bootstrap.
pub const SDKWORK_COMMUNITY_SNOWFLAKE_NODE_ID_ENV: &str = "SDKWORK_COMMUNITY_SNOWFLAKE_NODE_ID";

/// Escape hatch that re-enables the unsafe static node fallback outside
/// dev/test lifecycles.
pub const SDKWORK_COMMUNITY_ALLOW_UNSAFE_ID_FALLBACK_ENV: &str =
    "SDKWORK_COMMUNITY_ALLOW_UNSAFE_ID_FALLBACK";

pub struct CommunityServiceHost {
    database_pool: DatabasePool,
    service: Arc<CommunityService>,
    /// Keeps the database node lease heartbeat alive for the host lifetime;
    /// dropping the lease would release the snowflake node.
    _id_lease: Option<NodeLease>,
}

impl CommunityServiceHost {
    /// Boots the host from environment: PostgreSQL, the snowflake id
    /// generator, and the in-process commerce composition. Fails at startup
    /// when a split-deployment commerce backend is opted into without its
    /// credentials (`MODULE_SPEC.md` section 4.2).
    pub async fn from_env() -> Result<Self, String> {
        Self::from_env_with_membership_publisher(None).await
    }

    /// Same as [`Self::from_env`] but lets the composition root pass an
    /// already-built in-process membership package publisher. When it is not
    /// passed, the host composes the embedded membership adapter itself on the
    /// shared PostgreSQL pool, which is the default
    /// (`APPLICATION_GATEWAY_SPEC.md` section 2.3).
    pub async fn from_env_with_membership_publisher(
        membership_publisher: Option<Arc<dyn MembershipPackagePublisher>>,
    ) -> Result<Self, String> {
        let _ = dotenvy::dotenv();
        let database =
            sdkwork_community_storage_sqlx::bootstrap_community_database_from_env().await?;
        let pool = database.pool().clone();
        let (id_generator, id_lease) = build_id_generator(&pool).await?;
        let store = Arc::new(CommunitySqlxStore::new(pool.clone()));
        let service = Arc::new(build_service(
            &pool,
            store,
            id_generator,
            membership_publisher,
            None,
        )?);
        spawn_official_tier_publish_bootstrap(service.clone());
        Ok(Self {
            database_pool: pool,
            service,
            _id_lease: id_lease,
        })
    }

    pub async fn from_database_pool(pool: DatabasePool) -> Result<Arc<Self>, String> {
        Self::from_database_pool_with_commerce_ports(pool, None, None).await
    }

    /// Same as [`Self::from_database_pool`] but lets the composition root pass
    /// an already-built in-process membership package publisher; every unset
    /// port defaults to the embedded adapter on this pool.
    pub async fn from_database_pool_with_membership_publisher(
        pool: DatabasePool,
        membership_publisher: Option<Arc<dyn MembershipPackagePublisher>>,
    ) -> Result<Arc<Self>, String> {
        Self::from_database_pool_with_commerce_ports(pool, membership_publisher, None).await
    }

    /// Same as [`Self::from_database_pool`] but lets the composition root pass
    /// pre-built in-process commerce ports (membership package publisher and/or
    /// order payment verifier). Ports that are not passed default to the
    /// embedded sibling adapters on this pool; a dependency explicitly opted
    /// into a split deployment (`*_BACKEND_API_BASE_URL`) keeps its HTTP
    /// adapter instead (`MODULE_SPEC.md` section 4.2).
    pub async fn from_database_pool_with_commerce_ports(
        pool: DatabasePool,
        membership_publisher: Option<Arc<dyn MembershipPackagePublisher>>,
        order_verifier: Option<Arc<dyn OrderPaymentVerifier>>,
    ) -> Result<Arc<Self>, String> {
        let database = bootstrap_community_database(pool.clone()).await?;
        let pool = database.pool().clone();
        let (id_generator, id_lease) = build_id_generator(&pool).await?;
        let store = Arc::new(CommunitySqlxStore::new(pool.clone()));
        let service = Arc::new(build_service(
            &pool,
            store,
            id_generator,
            membership_publisher,
            order_verifier,
        )?);
        spawn_official_tier_publish_bootstrap(service.clone());
        Ok(Arc::new(Self {
            database_pool: pool,
            service,
            _id_lease: id_lease,
        }))
    }

    pub fn database_pool(&self) -> &DatabasePool {
        &self.database_pool
    }

    pub fn service(&self) -> Arc<CommunityService> {
        self.service.clone()
    }
}

/// Where one commerce dependency's in-process port comes from after the
/// startup transport resolution.
enum PortSelection<T: ?Sized> {
    /// The composition root already built the port on this pool.
    Wired(Arc<T>),
    /// No port was passed and the transport is in-process: build the embedded
    /// sibling adapter on this pool while the host boots.
    Embedded,
    /// The dependency runs as a split deployment over HTTP, so an in-process
    /// port would shadow the explicit opt-in and is left unused.
    Unused,
}

/// Resolves one dependency's in-process port source.
///
/// This is the single place that decides between an explicitly wired port, the
/// embedded default, and the split-deployment HTTP adapter
/// (`MODULE_SPEC.md` section 4.2). It is pure so the decision can be tested
/// without a database, and it can never silently downgrade a deliberate split
/// deployment to a local read.
fn select_port<T: ?Sized>(mode: CommerceTransportMode, wired: Option<Arc<T>>) -> PortSelection<T> {
    match (mode, wired) {
        (CommerceTransportMode::Http, _) => PortSelection::Unused,
        (CommerceTransportMode::InProcess, Some(port)) => PortSelection::Wired(port),
        (CommerceTransportMode::InProcess, None) => PortSelection::Embedded,
    }
}

/// Builds the community service with the in-process commerce composition
/// (`MODULE_SPEC.md` section 4.2).
///
/// The default transport is in-process: unless the operator explicitly opted
/// into a split deployment by setting a backend base URL, the embedded
/// membership and order adapters are composed on this process's PostgreSQL
/// pool. An explicitly wired port wins over nothing, but a dependency whose
/// transport is HTTP keeps the HTTP adapter so a deliberate split deployment
/// is never silently downgraded to a local read.
///
/// [`CommerceIntegration::from_env`] and [`CommerceIntegration::validate_ready`]
/// run while the host boots, so a backend base URL without its credentials, or
/// a missing embedded port, fails startup instead of the first tier publish or
/// membership activation.
fn build_service(
    pool: &DatabasePool,
    store: Arc<CommunitySqlxStore>,
    id_generator: Arc<dyn IdGenerator>,
    membership_publisher: Option<Arc<dyn MembershipPackagePublisher>>,
    order_verifier: Option<Arc<dyn OrderPaymentVerifier>>,
) -> Result<CommunityService, String> {
    // Startup validation: a configured HTTP base URL must already have both
    // dual-token credentials.
    let commerce = CommerceIntegration::from_env()?;

    // In-process default (and the only path when no base URL is set): consume
    // the embedded sibling repositories on the shared pool. When the operator
    // opted into HTTP for that dependency, keep the HTTP adapter.
    let membership_publisher = match select_port(commerce.membership_mode(), membership_publisher) {
        PortSelection::Unused => None,
        PortSelection::Wired(publisher) => Some(publisher),
        PortSelection::Embedded => {
            let publisher: Arc<dyn MembershipPackagePublisher> = Arc::new(
                sdkwork_community_commerce_membership_embedded::EmbeddedMembershipPackagePublisher::from_pool(
                    pool,
                )
                .map_err(|error| {
                    format!("embedded membership commerce publisher unavailable: {error}")
                })?,
            );
            Some(publisher)
        }
    };
    let order_verifier = match select_port(commerce.order_mode(), order_verifier) {
        PortSelection::Unused => None,
        PortSelection::Wired(verifier) => Some(verifier),
        PortSelection::Embedded => {
            let verifier: Arc<dyn OrderPaymentVerifier> = Arc::new(
                sdkwork_community_commerce_order_embedded::EmbeddedOrderPaymentVerifier::from_pool(
                    pool,
                )
                .map_err(|error| {
                    format!("embedded order commerce verifier unavailable: {error}")
                })?,
            );
            Some(verifier)
        }
    };

    let commerce = match membership_publisher {
        Some(publisher) => commerce.with_in_process_publisher(publisher),
        None => commerce,
    };
    let commerce = match order_verifier {
        Some(verifier) => commerce.with_in_process_order_verifier(verifier),
        None => commerce,
    };
    commerce.validate_ready()?;
    Ok(CommunityService::with_id_generator(
        store,
        Arc::new(commerce),
        id_generator,
    ))
}

/// Builds the backend-owned snowflake id generator.
///
/// Production prefers a database-allocated node from the shared
/// `sdkwork_node_registry` (collision-free across processes). When the
/// allocator is unavailable the host falls back to a static node
/// (`SDKWORK_COMMUNITY_SNOWFLAKE_NODE_ID`, then node 0) for dev/test, and
/// fails closed outside dev/test lifecycles.
async fn build_id_generator(
    pool: &DatabasePool,
) -> Result<(Arc<dyn IdGenerator>, Option<NodeLease>), String> {
    let config = NodeAllocatorConfig::from_service_name(COMMUNITY_SNOWFLAKE_SERVICE_NAME);
    match SnowflakeNodeAllocator::allocate_process_generator(pool, &config).await {
        Ok((generator, lease)) => Ok((Arc::new(generator), Some(lease))),
        Err(error) => {
            if id_fallback_is_forbidden() {
                return Err(format!(
                    "database snowflake node allocation failed for {COMMUNITY_SNOWFLAKE_SERVICE_NAME} \
                     and the unsafe static fallback is disabled: {error}"
                ));
            }
            let node_id = std::env::var(SDKWORK_COMMUNITY_SNOWFLAKE_NODE_ID_ENV)
                .ok()
                .and_then(|value| value.trim().parse::<u16>().ok())
                .unwrap_or(0);
            let generator =
                SnowflakeIdGenerator::new(node_id).map_err(|error| error.to_string())?;
            Ok((Arc::new(generator), None))
        }
    }
}

/// True when a static node fallback must not silently replace a failed
/// database allocation (mirrors the IM runtime-id policy).
fn id_fallback_is_forbidden() -> bool {
    let lifecycle = [
        "SDKWORK_COMMUNITY_ENVIRONMENT",
        "SDKWORK_IM_ENVIRONMENT",
        "SDKWORK_CLOUDROUTER_ENVIRONMENT",
    ]
    .into_iter()
    .find_map(|key| std::env::var(key).ok())
    .map(|value| value.trim().to_ascii_lowercase());
    if let Some(lifecycle) = lifecycle {
        return !matches!(lifecycle.as_str(), "development" | "dev" | "test");
    }
    let deployment_is_explicit = [
        "SDKWORK_COMMUNITY_DEPLOYMENT_PROFILE",
        "SDKWORK_IM_DEPLOYMENT_PROFILE",
        "SDKWORK_IM_RUNTIME_TARGET",
    ]
    .into_iter()
    .any(|key| std::env::var(key).is_ok_and(|value| !value.trim().is_empty()));
    if deployment_is_explicit {
        return true;
    }
    let explicit_override = std::env::var(SDKWORK_COMMUNITY_ALLOW_UNSAFE_ID_FALLBACK_ENV)
        .is_ok_and(|value| matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true"));
    !explicit_override && !cfg!(debug_assertions)
}

/// Publishes the official seeded circle tiers shortly after startup
/// (idempotent), retrying until the collapsed ingress listener is ready.
///
/// Fail-soft: a missing commerce backend (or any transient error) must never
/// block gateway boot; the next startup retries automatically.
fn spawn_official_tier_publish_bootstrap(service: Arc<CommunityService>) {
    // The auto-publish registers packages on the membership capability.
    // `build_service` already validated the transport at startup: the default
    // is the embedded in-process publisher on the shared pool, and HTTP is
    // only reachable through an explicit split-deployment opt-in. This guard
    // remains for hosts that construct the commerce integration themselves.
    if !service.commerce().membership_publish_is_configured() {
        eprintln!(
            "[community] official circle tier auto-publish skipped: the membership commerce \
             transport has no reachable implementation (compose the embedded in-process \
             publisher, or opt into a split deployment by configuring the membership backend \
             base URL and both dual-token credentials)"
        );
        return;
    }
    tokio::spawn(async move {
        for attempt in 1..=10u32 {
            match service.publish_official_circle_tiers().await {
                Ok(count) => {
                    if count > 0 {
                        eprintln!("[community] auto-published {count} official circle tier(s)");
                    }
                    return;
                }
                Err(error) => {
                    eprintln!(
                        "[community] official circle tier auto-publish attempt {attempt} failed: {}; retrying",
                        error.message()
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_community_service::{
        CommerceScope, MembershipPackagePublishFuture, MembershipPackageRegistration,
        OrderPaymentVerification, OrderPaymentVerifyFuture, RegisteredMembershipPackage,
    };

    struct StubPublisher;

    impl MembershipPackagePublisher for StubPublisher {
        fn register_membership_package<'a>(
            &'a self,
            registration: MembershipPackageRegistration,
            _scope: CommerceScope,
        ) -> MembershipPackagePublishFuture<'a> {
            Box::pin(async move {
                Ok(RegisteredMembershipPackage {
                    id: "package-1".to_owned(),
                    external_id: 1,
                    name: registration.name,
                })
            })
        }
    }

    struct StubVerifier;

    impl OrderPaymentVerifier for StubVerifier {
        fn verify_order_payment<'a>(
            &'a self,
            _order_id: &'a str,
            _scope: CommerceScope,
        ) -> OrderPaymentVerifyFuture<'a> {
            Box::pin(async move {
                Ok(OrderPaymentVerification {
                    paid: false,
                    paid_amount: None,
                })
            })
        }
    }

    #[test]
    fn a_split_deployment_dependency_never_falls_back_to_a_local_port() {
        let wired: Option<Arc<dyn MembershipPackagePublisher>> = Some(Arc::new(StubPublisher));
        assert!(matches!(
            select_port(CommerceTransportMode::Http, wired),
            PortSelection::Unused
        ));
        let wired: Option<Arc<dyn OrderPaymentVerifier>> = Some(Arc::new(StubVerifier));
        assert!(matches!(
            select_port(CommerceTransportMode::Http, wired),
            PortSelection::Unused
        ));
    }

    #[test]
    fn an_explicitly_wired_port_is_reused_instead_of_rebuilt() {
        let wired: Option<Arc<dyn MembershipPackagePublisher>> = Some(Arc::new(StubPublisher));
        assert!(matches!(
            select_port(CommerceTransportMode::InProcess, wired),
            PortSelection::Wired(_)
        ));
    }

    #[test]
    fn an_in_process_dependency_without_a_port_is_built_while_the_host_boots() {
        let none: Option<Arc<dyn MembershipPackagePublisher>> = None;
        assert!(matches!(
            select_port(CommerceTransportMode::InProcess, none),
            PortSelection::Embedded
        ));
        let none: Option<Arc<dyn OrderPaymentVerifier>> = None;
        assert!(matches!(
            select_port(CommerceTransportMode::InProcess, none),
            PortSelection::Embedded
        ));
    }
}
