//! Commerce integration: membership package registration and order payment
//! verification through the sdkwork-membership and sdkwork-order backend APIs.
//!
//! The community module never creates or settles orders itself; it registers
//! circle membership tiers as purchasable membership packages (so
//! sdkwork-order can resolve `packageId` by external id) and, after the
//! mobile cashier reports a paid order, verifies the order status with the
//! order backend before activating the circle membership.
//!
//! Backend surfaces are dual-token protected (Authorization bearer = auth
//! token, `Access-Token` header = access token), so each backend needs both
//! credentials configured:
//! `SDKWORK_MEMBERSHIP_BACKEND_AUTH_TOKEN` /
//! `SDKWORK_MEMBERSHIP_BACKEND_ACCESS_TOKEN` and
//! `SDKWORK_ORDER_BACKEND_AUTH_TOKEN` / `SDKWORK_ORDER_BACKEND_ACCESS_TOKEN`.

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::Serialize;

/// Canonical backend credential headers (dual-token; the internal ingress
/// token is used when the backend surface requires internal-api routing).
const ACCESS_TOKEN_HEADER: &str = "Access-Token";
const AUTHORIZATION_HEADER: &str = "Authorization";

#[derive(Debug, Clone)]
pub struct CommerceIntegrationConfig {
    pub membership_backend_base_url: Option<String>,
    pub membership_backend_auth_token: Option<String>,
    pub membership_backend_access_token: Option<String>,
    pub order_backend_base_url: Option<String>,
    pub order_backend_auth_token: Option<String>,
    pub order_backend_access_token: Option<String>,
}

impl CommerceIntegrationConfig {
    pub fn from_env() -> Self {
        Self {
            membership_backend_base_url: env("SDKWORK_MEMBERSHIP_BACKEND_API_BASE_URL"),
            membership_backend_auth_token: env("SDKWORK_MEMBERSHIP_BACKEND_AUTH_TOKEN"),
            membership_backend_access_token: env("SDKWORK_MEMBERSHIP_BACKEND_ACCESS_TOKEN"),
            order_backend_base_url: env("SDKWORK_ORDER_BACKEND_API_BASE_URL"),
            order_backend_auth_token: env("SDKWORK_ORDER_BACKEND_AUTH_TOKEN"),
            order_backend_access_token: env("SDKWORK_ORDER_BACKEND_ACCESS_TOKEN"),
        }
    }
}

fn env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MembershipPackageRegistration {
    /// Catalog classification of the registered package; circle tiers always
    /// register under the `community` plan family (membership backend
    /// requires the category and provisions referenced plans/groups with it).
    pub category: String,
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
/// membership backend capability (`APPLICATION_GATEWAY_SPEC.md` section 2.3).
///
/// Implementations:
/// - the built-in HTTP adapter in [`CommerceIntegration`] — for deployments
///   where the membership backend runs in a separate process (base URL and
///   dual-token credentials come from [`CommerceIntegrationConfig`]);
/// - in-process adapters wired by the deployment composition root — for
///   gateways that embed the membership assembly in the same process; these
///   consume the capability directly and `MUST NOT` loop HTTP requests back
///   through the gateway's own listener.
pub trait MembershipPackagePublisher: Send + Sync {
    fn register_membership_package<'a>(
        &'a self,
        registration: MembershipPackageRegistration,
    ) -> MembershipPackagePublishFuture<'a>;
}

/// Boxed future returned by [`OrderPaymentVerifier`].
pub type OrderPaymentVerifyFuture<'a> =
    Pin<Box<dyn Future<Output = Result<OrderPaymentVerification, String>> + Send + 'a>>;

/// Required port for verifying an order's payment state on the order backend
/// capability (`APPLICATION_GATEWAY_SPEC.md` section 2.3).
///
/// Implementations:
/// - the built-in HTTP adapter in [`CommerceIntegration`] — for deployments
///   where the order backend runs in a separate process (base URL and
///   dual-token credentials come from [`CommerceIntegrationConfig`]);
/// - in-process adapters wired by the deployment composition root — for
///   gateways that embed the order assembly in the same process; these
///   consume the capability directly and `MUST NOT` loop HTTP requests back
///   through the gateway's own listener.
pub trait OrderPaymentVerifier: Send + Sync {
    fn verify_order_payment<'a>(&'a self, order_id: &'a str) -> OrderPaymentVerifyFuture<'a>;
}

#[derive(Clone)]
pub struct CommerceIntegration {
    config: Arc<CommerceIntegrationConfig>,
    http: reqwest::Client,
    /// In-process publisher wired by the composition root for embedded
    /// same-process membership deployments. Takes precedence over the HTTP
    /// adapter; no per-dependency base URL or loopback IAM credentials are
    /// consulted when it is present.
    in_process_publisher: Option<Arc<dyn MembershipPackagePublisher>>,
    /// In-process order payment verifier wired by the composition root for
    /// embedded same-process order deployments. Takes precedence over the
    /// HTTP adapter (see [`OrderPaymentVerifier`]).
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
    pub fn new(config: CommerceIntegrationConfig) -> Self {
        Self {
            config: Arc::new(config),
            http: reqwest::Client::new(),
            in_process_publisher: None,
            in_process_order_verifier: None,
        }
    }

    /// Wires an in-process membership package publisher (embedded
    /// same-process deployment). When set, package registration consumes the
    /// embedded membership capability directly and never uses the HTTP
    /// adapter.
    pub fn with_in_process_publisher(
        mut self,
        publisher: Arc<dyn MembershipPackagePublisher>,
    ) -> Self {
        self.in_process_publisher = Some(publisher);
        self
    }

    /// Wires an in-process order payment verifier (embedded same-process
    /// deployment). When set, payment verification consumes the embedded
    /// order capability directly and never uses the HTTP adapter.
    pub fn with_in_process_order_verifier(
        mut self,
        verifier: Arc<dyn OrderPaymentVerifier>,
    ) -> Self {
        self.in_process_order_verifier = Some(verifier);
        self
    }

    /// True when payment verification can run: an in-process order verifier
    /// is wired, or the HTTP order backend (base URL plus dual tokens) is
    /// fully configured.
    pub fn order_verify_is_configured(&self) -> bool {
        self.in_process_order_verifier.is_some()
            || (self.config.order_backend_base_url.is_some()
                && self.config.order_backend_auth_token.is_some()
                && self.config.order_backend_access_token.is_some())
    }

    /// True when tier publishing can run: an in-process publisher is wired,
    /// or the HTTP membership backend (base URL plus dual tokens) is fully
    /// configured.
    pub fn membership_publish_is_configured(&self) -> bool {
        self.in_process_publisher.is_some()
            || (self.config.membership_backend_base_url.is_some()
                && self.config.membership_backend_auth_token.is_some()
                && self.config.membership_backend_access_token.is_some())
    }

    pub fn config(&self) -> &CommerceIntegrationConfig {
        &self.config
    }

    /// Dual-token backend headers: the auth token goes in the Authorization
    /// bearer, the access token in the `Access-Token` header. Backend routes
    /// resolve the paired IAM session from both credentials.
    fn backend_headers(&self, auth_token: &str, access_token: &str) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Ok(value) = reqwest::header::HeaderValue::from_str(access_token) {
            headers.insert(ACCESS_TOKEN_HEADER, value);
        }
        if let Ok(value) = reqwest::header::HeaderValue::from_str(auth_token) {
            let bearer = format!("Bearer {auth_token}");
            if let Ok(value) = reqwest::header::HeaderValue::from_str(&bearer) {
                headers.insert(AUTHORIZATION_HEADER, value);
            } else if let Ok(value) = reqwest::header::HeaderValue::from_str(auth_token) {
                headers.insert(AUTHORIZATION_HEADER, value);
            }
        }
        headers
    }

    /// Registers a purchasable membership package on the membership backend
    /// and returns the assigned external id (the `packageId` sdkwork-order
    /// resolves when creating a membership order).
    pub async fn register_membership_package(
        &self,
        registration: MembershipPackageRegistration,
    ) -> Result<RegisteredMembershipPackage, String> {
        // Embedded same-process deployment: consume the mounted membership
        // capability directly through the wired in-process port; never loop
        // back through the gateway's own HTTP listener.
        if let Some(publisher) = self.in_process_publisher.as_ref() {
            return publisher.register_membership_package(registration).await;
        }
        let base_url = self
            .config
            .membership_backend_base_url
            .clone()
            .ok_or_else(|| {
                "membership backend is not configured (SDKWORK_MEMBERSHIP_BACKEND_API_BASE_URL)"
                    .to_owned()
            })?;
        let auth_token = self
            .config
            .membership_backend_auth_token
            .clone()
            .ok_or_else(|| {
                "membership backend credential is not configured (SDKWORK_MEMBERSHIP_BACKEND_AUTH_TOKEN)"
                    .to_owned()
            })?;
        let access_token = self
            .config
            .membership_backend_access_token
            .clone()
            .ok_or_else(|| {
                "membership backend credential is not configured (SDKWORK_MEMBERSHIP_BACKEND_ACCESS_TOKEN)"
                    .to_owned()
            })?;

        let url = format!(
            "{}/backend/v3/api/memberships/packages",
            base_url.trim_end_matches('/')
        );
        let response = self
            .http
            .post(&url)
            .headers(self.backend_headers(&auth_token, &access_token))
            .json(&registration)
            .send()
            .await
            .map_err(|error| format!("membership package registration request failed: {error}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!(
                "membership package registration rejected ({status}): {}",
                body.chars().take(300).collect::<String>()
            ));
        }

        let payload: serde_json::Value = response.json().await.map_err(|error| {
            format!("membership package registration response parse failed: {error}")
        })?;
        // Envelope shape: `{ code, data: { item: { id, externalId, ... } } }`.
        let item = payload
            .get("data")
            .and_then(|data| data.get("item"))
            .or_else(|| payload.get("data"))
            .unwrap_or(&payload);
        let external_id = item
            .get("externalId")
            .or_else(|| item.get("external_id"))
            .and_then(|value| {
                value.as_i64().or_else(|| {
                    value
                        .as_str()
                        .and_then(|raw| raw.trim().parse::<i64>().ok())
                })
            })
            .ok_or_else(|| {
                "membership package registration did not return externalId".to_owned()
            })?;
        Ok(RegisteredMembershipPackage {
            id: item
                .get("id")
                .and_then(|value| value.as_str())
                .unwrap_or_default()
                .to_owned(),
            external_id,
            name: item
                .get("name")
                .and_then(|value| value.as_str())
                .unwrap_or_default()
                .to_owned(),
        })
    }

    /// Verifies that an order has been paid by querying the order backend and
    /// returns the paid amount (when present) so the community can accumulate
    /// the actual paid revenue instead of a possibly stale tier price.
    pub async fn verify_order_paid(
        &self,
        order_id: &str,
    ) -> Result<OrderPaymentVerification, String> {
        // Embedded same-process deployment: consume the mounted order
        // capability directly through the wired in-process port; never loop
        // back through the gateway's own HTTP listener.
        if let Some(verifier) = self.in_process_order_verifier.as_ref() {
            return verifier.verify_order_payment(order_id).await;
        }
        let base_url = self.config.order_backend_base_url.clone().ok_or_else(|| {
            "order backend is not configured (SDKWORK_ORDER_BACKEND_API_BASE_URL)".to_owned()
        })?;
        let auth_token = self
            .config
            .order_backend_auth_token
            .clone()
            .ok_or_else(|| {
                "order backend credential is not configured (SDKWORK_ORDER_BACKEND_AUTH_TOKEN)"
                    .to_owned()
            })?;
        let access_token = self
            .config
            .order_backend_access_token
            .clone()
            .ok_or_else(|| {
                "order backend credential is not configured (SDKWORK_ORDER_BACKEND_ACCESS_TOKEN)"
                    .to_owned()
            })?;

        let url = format!(
            "{}/backend/v3/api/orders/{}",
            base_url.trim_end_matches('/'),
            urlencode(order_id)
        );
        let response = self
            .http
            .get(&url)
            .headers(self.backend_headers(&auth_token, &access_token))
            .send()
            .await
            .map_err(|error| format!("order verification request failed: {error}"))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(OrderPaymentVerification {
                paid: false,
                paid_amount: None,
            });
        }
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!(
                "order verification rejected ({status}): {}",
                body.chars().take(300).collect::<String>()
            ));
        }

        let payload: serde_json::Value = response
            .json()
            .await
            .map_err(|error| format!("order verification response parse failed: {error}"))?;
        // Envelope shape: `{ code, data: { item: { status, totalAmount, ... } } }`.
        let item = payload
            .get("data")
            .and_then(|data| data.get("item"))
            .or_else(|| payload.get("data"))
            .unwrap_or(&payload);
        let status = item
            .get("status")
            .and_then(|value| value.as_str())
            .unwrap_or_default()
            .to_lowercase();
        let paid = is_paid_order_status(&status);
        let paid_amount = item
            .get("paidAmount")
            .or_else(|| item.get("totalAmount"))
            .and_then(|value| value.as_str())
            .and_then(|value| value.parse::<f64>().ok());
        Ok(OrderPaymentVerification { paid, paid_amount })
    }
}

fn urlencode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char)
            }
            _ => {
                encoded.push_str(&format!("%{byte:02X}"));
            }
        }
    }
    encoded
}

/// Order statuses that indicate a settled, paid order (order backend uses
/// `paid` / `completed` style states; the app API payment-success query
/// reports `paid: true` for the same transitions).
fn is_paid_order_status(status: &str) -> bool {
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
}
