//! Commerce integration: circle membership package registration and order
//! payment verification against the sdkwork-membership and sdkwork-order
//! modules.
//!
//! The community module never creates or settles orders itself; it registers
//! circle membership tiers as purchasable membership packages (so
//! sdkwork-order can resolve `packageId` by external id) and, after the mobile
//! cashier reports a paid order, verifies the order status before activating
//! the circle membership.
//!
//! # Transport boundary (`MODULE_SPEC.md` section 4.2)
//!
//! The packaged deployment runs one backend process against one PostgreSQL, so
//! the community module reaches membership and order **in-process**: the
//! composition root wires the embedded sibling repositories on the shared pool
//! and this module calls them through the [`MembershipPackagePublisher`] and
//! [`OrderPaymentVerifier`] ports. That is the default and needs no endpoint or
//! credential configuration.
//!
//! HTTP exists only as a deliberate **split deployment** opt-in: setting
//! `SDKWORK_MEMBERSHIP_BACKEND_API_BASE_URL` or
//! `SDKWORK_ORDER_BACKEND_API_BASE_URL` selects the adapter in
//! [`super::commerce_http`], and [`CommerceIntegrationConfig::from_env`]
//! validates the matching dual-token credentials **at startup**. A
//! half-configured split deployment therefore fails while the host boots, not
//! on the first tier publish or membership activation after state has changed —
//! the production coupon-incident failure class.
//!
//! Both transports preserve:
//!
//! - **tenant/organization scope** — callers pass a [`CommerceScope`]; the
//!   in-process adapters scope the sibling query/command with it, while the HTTP
//!   adapter keeps the scope its backend credentials already carry;
//! - **idempotency** — the membership package `code` (stable per tier, and for
//!   the lifetime variant) remains the idempotency key; the in-process
//!   repository upserts on it and derives its request id from it, so a retried
//!   publish returns the same `externalId`;
//! - **DTO mapping** — the same [`MembershipPackageRegistration`] /
//!   [`RegisteredMembershipPackage`] / [`OrderPaymentVerification`] shapes;
//! - **error mapping** — every failure surfaces to the service layer as
//!   `CommunityServiceError::Integration`, exactly as the HTTP adapter did, so
//!   the response stays `500 internal` and no public status code changes.

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::Serialize;

use super::commerce_http::{self, CommerceHttpBackend};

/// Tenant/organization scope of one commerce call.
///
/// The community service resolves it from the tenant that owns the tier or the
/// membership; in-process adapters use it to scope the sibling
/// query/command. Split-deployment HTTP adapters authenticate as a backend
/// service account, so their scope comes from those credentials and this value
/// is not sent on the wire.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommerceScope {
    tenant_id: Option<String>,
    organization_id: Option<String>,
}

impl CommerceScope {
    /// Scopes a call to one tenant.
    #[must_use]
    pub fn new(tenant_id: impl Into<String>) -> Self {
        Self {
            tenant_id: non_empty(tenant_id.into()),
            organization_id: None,
        }
    }

    /// Adds an organization scope (`None` means tenant-level).
    #[must_use]
    pub fn with_organization_id(mut self, organization_id: impl Into<String>) -> Self {
        self.organization_id = non_empty(organization_id.into());
        self
    }

    /// The scoped tenant, when the caller knows it.
    #[must_use]
    pub fn tenant_id(&self) -> Option<&str> {
        self.tenant_id.as_deref()
    }

    /// The scoped organization, when the caller knows it.
    #[must_use]
    pub fn organization_id(&self) -> Option<&str> {
        self.organization_id.as_deref()
    }
}

fn non_empty(value: String) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_owned())
}

/// How one commerce dependency is reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommerceTransportMode {
    /// In-process: the composition root wired the embedded sibling capability.
    InProcess,
    /// Explicit split-deployment opt-in: HTTP to a separately deployed module.
    Http,
}

impl CommerceTransportMode {
    /// Stable spelling for logs and diagnostics.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::InProcess => "in-process",
            Self::Http => "http",
        }
    }
}

/// One dependency's selected transport.
#[derive(Clone, Default)]
enum CommerceBackend {
    #[default]
    InProcess,
    Http(CommerceHttpBackend),
}

impl CommerceBackend {
    fn mode(&self) -> CommerceTransportMode {
        match self {
            Self::InProcess => CommerceTransportMode::InProcess,
            Self::Http(_) => CommerceTransportMode::Http,
        }
    }

    fn http(&self) -> Option<&CommerceHttpBackend> {
        match self {
            Self::InProcess => None,
            Self::Http(backend) => Some(backend),
        }
    }
}

/// Transport selection for the two commerce dependencies.
///
/// Build it with [`CommerceIntegrationConfig::from_env`] at host startup (the
/// only constructor that may opt into HTTP) or with
/// [`CommerceIntegrationConfig::in_process`] for tests and embedded hosts.
#[derive(Clone, Default)]
pub struct CommerceIntegrationConfig {
    membership: CommerceBackend,
    order: CommerceBackend,
}

impl CommerceIntegrationConfig {
    /// In-process for both dependencies. This is the default deployment shape
    /// and the constructor used by tests.
    #[must_use]
    pub fn in_process() -> Self {
        Self::default()
    }

    /// Resolves the transport from the environment.
    ///
    /// An unset (or blank) backend base URL keeps the in-process default. A set
    /// base URL is the explicit HTTP opt-in and **requires** both backend
    /// credentials; otherwise this returns an error so the host fails at
    /// startup (MODULE_SPEC.md section 4.2).
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            membership: match commerce_http::membership_backend_from_env()? {
                Some(backend) => CommerceBackend::Http(backend),
                None => CommerceBackend::InProcess,
            },
            order: match commerce_http::order_backend_from_env()? {
                Some(backend) => CommerceBackend::Http(backend),
                None => CommerceBackend::InProcess,
            },
        })
    }

    /// Selected membership transport.
    #[must_use]
    pub fn membership_mode(&self) -> CommerceTransportMode {
        self.membership.mode()
    }

    /// Selected order transport.
    #[must_use]
    pub fn order_mode(&self) -> CommerceTransportMode {
        self.order.mode()
    }

    /// True when membership runs through the split-deployment HTTP adapter.
    #[must_use]
    pub fn membership_uses_http(&self) -> bool {
        self.membership_mode() == CommerceTransportMode::Http
    }

    /// True when order runs through the split-deployment HTTP adapter.
    #[must_use]
    pub fn order_uses_http(&self) -> bool {
        self.order_mode() == CommerceTransportMode::Http
    }

    fn membership_http(&self) -> Option<&CommerceHttpBackend> {
        self.membership.http()
    }

    fn order_http(&self) -> Option<&CommerceHttpBackend> {
        self.order.http()
    }
}

impl fmt::Debug for CommerceIntegrationConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Endpoint URLs and tokens never enter logs.
        formatter
            .debug_struct("CommerceIntegrationConfig")
            .field("membership", &self.membership_mode())
            .field("order", &self.order_mode())
            .finish()
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MembershipPackageRegistration {
    /// Catalog classification of the registered package; circle tiers always
    /// register under the `community` plan family (membership backend
    /// requires the category and provisions referenced plans/groups with it).
    pub category: String,
    /// Stable package code. This is the idempotency key for registration: both
    /// transports resolve a retried registration to the same package row and
    /// `externalId`.
    pub code: String,
    pub package_group_id: String,
    pub plan_id: String,
    pub name: String,
    pub price_amount: String,
    pub currency_code: String,
    /// Membership backend duration in days (int64 wire contract: string).
    #[serde(with = "sdkwork_utils_rust::serde_int64")]
    pub duration_days: i64,
    /// Membership backend discount percent (1-100); 100 means full price.
    pub discount: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredMembershipPackage {
    pub id: String,
    pub external_id: i64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct OrderPaymentVerification {
    pub paid: bool,
    pub paid_amount: Option<f64>,
}

/// Boxed future returned by [`MembershipPackagePublisher`].
pub type MembershipPackagePublishFuture<'a> =
    Pin<Box<dyn Future<Output = Result<RegisteredMembershipPackage, String>> + Send + 'a>>;

/// Required port for registering a purchasable membership package on the
/// membership module capability (`APPLICATION_GATEWAY_SPEC.md` section 2.3).
///
/// Implementations:
/// - the embedded in-process publisher (`sdkwork-community-commerce-membership-embedded`)
///   — the default; it shares the process, the PostgreSQL pool, and the
///   transaction/error semantics of the membership module;
/// - the split-deployment HTTP adapter in [`super::commerce_http`], selected
///   only when the membership backend base URL is explicitly configured.
///
/// Implementations must treat `registration.code` as the idempotency key.
pub trait MembershipPackagePublisher: Send + Sync {
    fn register_membership_package<'a>(
        &'a self,
        registration: MembershipPackageRegistration,
        scope: CommerceScope,
    ) -> MembershipPackagePublishFuture<'a>;
}

/// Boxed future returned by [`OrderPaymentVerifier`].
pub type OrderPaymentVerifyFuture<'a> =
    Pin<Box<dyn Future<Output = Result<OrderPaymentVerification, String>> + Send + 'a>>;

/// Required port for verifying an order's payment state on the order module
/// capability (`APPLICATION_GATEWAY_SPEC.md` section 2.3).
///
/// Implementations:
/// - the embedded in-process verifier (`sdkwork-community-commerce-order-embedded`)
///   — the default; it reads the order through its public repository query;
/// - the split-deployment HTTP adapter in [`super::commerce_http`], selected
///   only when the order backend base URL is explicitly configured.
///
/// A missing/unknown order is `paid: false`, never an error, in both
/// transports.
pub trait OrderPaymentVerifier: Send + Sync {
    fn verify_order_payment<'a>(
        &'a self,
        order_id: &'a str,
        scope: CommerceScope,
    ) -> OrderPaymentVerifyFuture<'a>;
}

pub struct CommerceIntegration {
    config: Arc<CommerceIntegrationConfig>,
    /// In-process membership publisher wired by the composition root. Used when
    /// the membership transport is in-process.
    in_process_publisher: Option<Arc<dyn MembershipPackagePublisher>>,
    /// In-process order payment verifier wired by the composition root. Used
    /// when the order transport is in-process.
    in_process_order_verifier: Option<Arc<dyn OrderPaymentVerifier>>,
}

impl fmt::Debug for CommerceIntegration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CommerceIntegration")
            .field("config", &self.config)
            .field(
                "in_process_publisher",
                &self.in_process_publisher.as_ref().map(|_| "<wired>"),
            )
            .field(
                "in_process_order_verifier",
                &self.in_process_order_verifier.as_ref().map(|_| "<wired>"),
            )
            .finish()
    }
}

impl CommerceIntegration {
    /// Builds the integration on an already resolved config.
    pub fn new(config: CommerceIntegrationConfig) -> Self {
        Self {
            config: Arc::new(config),
            in_process_publisher: None,
            in_process_order_verifier: None,
        }
    }

    /// Builds the integration from the environment, failing `Err` when a
    /// split-deployment backend is opted into without its credentials. Call
    /// this while the host boots.
    pub fn from_env() -> Result<Self, String> {
        Ok(Self::new(CommerceIntegrationConfig::from_env()?))
    }

    /// Wires the in-process membership package publisher (the default
    /// composition). Used whenever the membership transport is in-process.
    #[must_use]
    pub fn with_in_process_publisher(
        mut self,
        publisher: Arc<dyn MembershipPackagePublisher>,
    ) -> Self {
        self.in_process_publisher = Some(publisher);
        self
    }

    /// Wires the in-process order payment verifier (the default composition).
    /// Used whenever the order transport is in-process.
    #[must_use]
    pub fn with_in_process_order_verifier(
        mut self,
        verifier: Arc<dyn OrderPaymentVerifier>,
    ) -> Self {
        self.in_process_order_verifier = Some(verifier);
        self
    }

    /// True when payment verification can run: the configured transport has a
    /// reachable implementation.
    #[must_use]
    pub fn order_verify_is_configured(&self) -> bool {
        match self.config.order_mode() {
            CommerceTransportMode::Http => self.config.order_http().is_some(),
            CommerceTransportMode::InProcess => self.in_process_order_verifier.is_some(),
        }
    }

    /// True when tier publishing can run: the configured transport has a
    /// reachable implementation.
    #[must_use]
    pub fn membership_publish_is_configured(&self) -> bool {
        match self.config.membership_mode() {
            CommerceTransportMode::Http => self.config.membership_http().is_some(),
            CommerceTransportMode::InProcess => self.in_process_publisher.is_some(),
        }
    }

    /// Startup check: every in-process dependency must have its embedded port
    /// wired. Split-deployment HTTP dependencies were already validated when
    /// the config was resolved.
    pub fn validate_ready(&self) -> Result<(), String> {
        if self.config.membership_mode() == CommerceTransportMode::InProcess
            && self.in_process_publisher.is_none()
        {
            return Err(
                "community commerce membership transport is in-process but no in-process \
                 MembershipPackagePublisher is wired; compose the embedded membership adapter \
                 on the shared PostgreSQL pool, or opt into a split deployment with the \
                 membership backend base URL and both dual-token credentials"
                    .to_owned(),
            );
        }
        if self.config.order_mode() == CommerceTransportMode::InProcess
            && self.in_process_order_verifier.is_none()
        {
            return Err(
                "community commerce order transport is in-process but no in-process \
                 OrderPaymentVerifier is wired; compose the embedded order adapter on the \
                 shared PostgreSQL pool, or opt into a split deployment with the order \
                 backend base URL and both dual-token credentials"
                    .to_owned(),
            );
        }
        Ok(())
    }

    /// Selected membership transport.
    #[must_use]
    pub fn membership_mode(&self) -> CommerceTransportMode {
        self.config.membership_mode()
    }

    /// Selected order transport.
    #[must_use]
    pub fn order_mode(&self) -> CommerceTransportMode {
        self.config.order_mode()
    }

    pub fn config(&self) -> &CommerceIntegrationConfig {
        &self.config
    }

    /// Registers a purchasable membership package and returns the assigned
    /// external id (the `packageId` sdkwork-order resolves when creating a
    /// membership order).
    pub async fn register_membership_package(
        &self,
        registration: MembershipPackageRegistration,
        scope: CommerceScope,
    ) -> Result<RegisteredMembershipPackage, String> {
        match self.config.membership_mode() {
            CommerceTransportMode::InProcess => {
                // Embedded same-process deployment: consume the mounted
                // membership repository directly through the wired port.
                let publisher = self.in_process_publisher.as_ref().ok_or_else(|| {
                    "in-process membership package publisher is not wired".to_owned()
                })?;
                publisher
                    .register_membership_package(registration, scope)
                    .await
            }
            CommerceTransportMode::Http => {
                let backend = self.config.membership_http().ok_or_else(|| {
                    "split-deployment membership backend is not configured".to_owned()
                })?;
                commerce_http::register_membership_package(backend, registration).await
            }
        }
    }

    /// Verifies that an order has been paid and returns the paid amount (when
    /// present) so the community can accumulate the actual paid revenue
    /// instead of a possibly stale tier price.
    pub async fn verify_order_paid(
        &self,
        order_id: &str,
        scope: CommerceScope,
    ) -> Result<OrderPaymentVerification, String> {
        match self.config.order_mode() {
            CommerceTransportMode::InProcess => {
                // Embedded same-process deployment: read the order through its
                // public in-process port; never loop back over HTTP.
                let verifier = self
                    .in_process_order_verifier
                    .as_ref()
                    .ok_or_else(|| "in-process order payment verifier is not wired".to_owned())?;
                verifier.verify_order_payment(order_id, scope).await
            }
            CommerceTransportMode::Http => {
                let backend = self
                    .config
                    .order_http()
                    .ok_or_else(|| "split-deployment order backend is not configured".to_owned())?;
                commerce_http::verify_order_payment(backend, order_id).await
            }
        }
    }
}

/// Order statuses that indicate a settled, paid order (the order module uses
/// `paid` / `completed` style states). Shared by both transports so the
/// in-process and HTTP results cannot drift.
#[must_use]
pub fn is_paid_order_status(status: &str) -> bool {
    matches!(
        status,
        "paid"
            | "completed"
            | "settled"
            | "success"
            | "finished"
            | "payment_success"
            | "payment-success"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn membership_package_registration_serializes_contract_shapes() {
        let registration = MembershipPackageRegistration {
            category: "community".to_owned(),
            code: "community-tier-official-llmstandard".to_owned(),
            package_group_id: "package-group-circle-membership".to_owned(),
            plan_id: "plan-circle-membership".to_owned(),
            name: "AI 大模型实战进阶 · 普通会员".to_owned(),
            price_amount: "199.00".to_owned(),
            currency_code: "CNY".to_owned(),
            duration_days: 365,
            discount: 100,
            status: "active".to_owned(),
        };
        let json = serde_json::to_value(&registration).expect("serialize registration");
        assert_eq!(json["category"], "community");
        // int64 wire contract (API_SPEC §13.6): durationDays must serialize
        // as a decimal string, not a JSON number.
        assert_eq!(json["durationDays"], "365");
        assert_eq!(json["discount"], 100);
        assert_eq!(json["priceAmount"], "199.00");
        assert_eq!(json["packageGroupId"], "package-group-circle-membership");
        assert_eq!(json["planId"], "plan-circle-membership");
        assert_eq!(json["status"], "active");
    }

    #[test]
    fn in_process_is_the_default_transport() {
        let config = CommerceIntegrationConfig::in_process();
        assert_eq!(config.membership_mode(), CommerceTransportMode::InProcess);
        assert_eq!(config.order_mode(), CommerceTransportMode::InProcess);
        assert!(!config.membership_uses_http());
        assert!(!config.order_uses_http());
        assert_eq!(CommerceTransportMode::InProcess.label(), "in-process");
        assert_eq!(CommerceTransportMode::Http.label(), "http");
    }

    #[test]
    fn an_unwired_in_process_dependency_is_a_startup_error() {
        let integration = CommerceIntegration::new(CommerceIntegrationConfig::in_process());
        let error = integration
            .validate_ready()
            .expect_err("in-process without a port must fail startup");
        assert!(error.contains("in-process"), "{error}");
        assert!(error.contains("MembershipPackagePublisher"), "{error}");
    }

    #[tokio::test]
    async fn an_in_process_publisher_dispatch_never_touches_http() {
        struct RecordingPublisher {
            seen: Arc<Mutex<Vec<(String, CommerceScope)>>>,
        }
        impl MembershipPackagePublisher for RecordingPublisher {
            fn register_membership_package<'a>(
                &'a self,
                registration: MembershipPackageRegistration,
                scope: CommerceScope,
            ) -> MembershipPackagePublishFuture<'a> {
                Box::pin(async move {
                    self.seen
                        .lock()
                        .expect("recording publisher mutex")
                        .push((registration.code, scope));
                    Ok(RegisteredMembershipPackage {
                        id: "package-1".to_owned(),
                        external_id: 42,
                        name: registration.name,
                    })
                })
            }
        }

        let seen = Arc::new(Mutex::new(Vec::new()));
        let integration = CommerceIntegration::new(CommerceIntegrationConfig::in_process())
            .with_in_process_publisher(Arc::new(RecordingPublisher { seen: seen.clone() }));
        assert!(integration.membership_publish_is_configured());

        let result = integration
            .register_membership_package(
                test_registration("community-tier-1"),
                CommerceScope::new("100001").with_organization_id("0"),
            )
            .await
            .expect("in-process registration");
        assert_eq!(result.external_id, 42);
        assert_eq!(result.id, "package-1");
        let seen = seen.lock().expect("recording publisher mutex");
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, "community-tier-1");
        assert_eq!(seen[0].1.tenant_id(), Some("100001"));
        assert_eq!(seen[0].1.organization_id(), Some("0"));
    }

    #[tokio::test]
    async fn an_in_process_verifier_maps_a_missing_order_to_unpaid() {
        struct MissingOrderVerifier;
        impl OrderPaymentVerifier for MissingOrderVerifier {
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

        let integration = CommerceIntegration::new(CommerceIntegrationConfig::in_process())
            .with_in_process_order_verifier(Arc::new(MissingOrderVerifier));
        assert!(integration.order_verify_is_configured());
        let verification = integration
            .verify_order_paid("order-1", CommerceScope::new("100001"))
            .await
            .expect("in-process verification");
        assert!(!verification.paid);
        assert_eq!(verification.paid_amount, None);
    }

    #[test]
    fn the_default_path_has_no_http_opt_in() {
        // Regression guard for MODULE_SPEC.md section 4.2: the default config
        // must not be HTTP, and `new` must not read the environment.
        let source = include_str!("commerce.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("production section");
        assert!(!production.contains("env::var("));
        assert!(production.contains("fn from_env()"));
        // Both dispatch methods must handle the in-process transport before
        // the HTTP opt-in branch.
        for (start, end) in [
            (
                "pub async fn register_membership_package",
                "pub async fn verify_order_paid",
            ),
            ("pub async fn verify_order_paid", "fn is_paid_order_status"),
        ] {
            let start = production.find(start).expect("dispatch entrypoint");
            let end = production.find(end).expect("dispatch entrypoint end");
            let dispatch = &production[start..end];
            let in_process_branch = dispatch
                .find("CommerceTransportMode::InProcess =>")
                .expect("in-process dispatch branch");
            let http_branch = dispatch
                .find("CommerceTransportMode::Http =>")
                .expect("http dispatch branch");
            assert!(in_process_branch < http_branch);
        }
    }

    #[test]
    fn the_paid_status_policy_is_shared_by_both_transports() {
        for status in [
            "paid",
            "completed",
            "settled",
            "success",
            "finished",
            "payment_success",
            "payment-success",
        ] {
            assert!(is_paid_order_status(status), "{status} must be paid");
        }
        for status in ["pending", "created", "cancelled", "refunded", ""] {
            assert!(!is_paid_order_status(status), "{status} must not be paid");
        }
    }

    fn test_registration(code: &str) -> MembershipPackageRegistration {
        MembershipPackageRegistration {
            category: "community".to_owned(),
            code: code.to_owned(),
            package_group_id: "package-group-circle-membership".to_owned(),
            plan_id: "plan-circle-membership".to_owned(),
            name: "Test tier".to_owned(),
            price_amount: "9.90".to_owned(),
            currency_code: "CNY".to_owned(),
            duration_days: 30,
            discount: 100,
            status: "active".to_owned(),
        }
    }
}
