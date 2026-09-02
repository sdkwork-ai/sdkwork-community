use std::sync::Arc;

use sdkwork_community_database_host::bootstrap_community_database;
pub use sdkwork_community_service::MembershipPackagePublisher;
pub use sdkwork_community_service::OrderPaymentVerifier;
use sdkwork_community_service::{CommerceIntegration, CommerceIntegrationConfig, CommunityService};
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
    pub async fn from_env() -> Result<Self, String> {
        Self::from_env_with_membership_publisher(None).await
    }

    /// Same as [`Self::from_env`] but wires an in-process membership package
    /// publisher for embedded same-process membership deployments
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
            store,
            id_generator,
            membership_publisher,
            None,
        ));
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

    /// Same as [`Self::from_database_pool`] but wires an in-process membership
    /// package publisher for embedded same-process membership deployments
    /// (`APPLICATION_GATEWAY_SPEC.md` section 2.3).
    pub async fn from_database_pool_with_membership_publisher(
        pool: DatabasePool,
        membership_publisher: Option<Arc<dyn MembershipPackagePublisher>>,
    ) -> Result<Arc<Self>, String> {
        Self::from_database_pool_with_commerce_ports(pool, membership_publisher, None).await
    }

    /// Same as [`Self::from_database_pool`] but wires the in-process commerce
    /// ports (membership package publisher and/or order payment verifier) for
    /// embedded same-process dependency deployments
    /// (`APPLICATION_GATEWAY_SPEC.md` section 2.3). Ports wired here consume
    /// the embedded dependency capability directly; unset ports fall back to
    /// the HTTP adapters configured through `CommerceIntegrationConfig`.
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
            store,
            id_generator,
            membership_publisher,
            order_verifier,
        ));
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

/// Builds the community service with the environment-configured commerce
/// integration, optionally overriding the HTTP membership/order adapters with
/// in-process ports wired by the deployment composition root
/// (`APPLICATION_GATEWAY_SPEC.md` section 2.3).
fn build_service(
    store: Arc<CommunitySqlxStore>,
    id_generator: Arc<dyn IdGenerator>,
    membership_publisher: Option<Arc<dyn MembershipPackagePublisher>>,
    order_verifier: Option<Arc<dyn OrderPaymentVerifier>>,
) -> CommunityService {
    let commerce = CommerceIntegration::new(CommerceIntegrationConfig::from_env());
    let commerce = match membership_publisher {
        Some(publisher) => commerce.with_in_process_publisher(publisher),
        None => commerce,
    };
    let commerce = match order_verifier {
        Some(verifier) => commerce.with_in_process_order_verifier(verifier),
        None => commerce,
    };
    CommunityService::with_id_generator(store, Arc::new(commerce), id_generator)
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
    // The auto-publish registers packages on the membership backend
    // capability. Publishing requires either an in-process publisher wired by
    // the composition root (embedded deployment) or a fully configured HTTP
    // membership backend (separate-process deployment). A missing
    // configuration is permanent, so log once and skip instead of emitting
    // repeated failed attempts during startup.
    if !service.commerce().membership_publish_is_configured() {
        eprintln!(
            "[community] official circle tier auto-publish skipped: membership commerce integration is not configured \
             (embed the membership assembly with an in-process publisher, or set SDKWORK_MEMBERSHIP_BACKEND_API_BASE_URL \
             with SDKWORK_MEMBERSHIP_BACKEND_AUTH_TOKEN and SDKWORK_MEMBERSHIP_BACKEND_ACCESS_TOKEN for a \
             separate-process membership deployment)"
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
