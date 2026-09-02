//! In-process order payment verification adapter
//! (`APPLICATION_GATEWAY_SPEC.md` section 2.3).
//!
//! Implements the community `OrderPaymentVerifier` required port against the
//! order repository embedded in the same gateway process, so the community
//! commerce integration consumes the mounted order capability directly
//! instead of looping HTTP requests back through the gateway's own listener.
//! The HTTP adapter (dual-token protected backend surface) remains the
//! contract for separate-process deployments and every real HTTP client.
//!
//! The service tenant used for management-order lookups is deployment
//! configuration (environment overrides) with development defaults; it scopes
//! the embedded repository query the same way the backend operator subject
//! scopes the HTTP surface.

use sdkwork_community_service::{
    OrderPaymentVerification, OrderPaymentVerifier, OrderPaymentVerifyFuture,
};
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_order_repository_sqlx::PostgresCommerceOrderStore;
use sdkwork_order_service::OrderManagementDetailQuery;

/// Overrides the platform service tenant id used for embedded order
/// management lookups. Defaults to `1`.
pub const SDKWORK_COMMUNITY_ORDER_SERVICE_TENANT_ID_ENV: &str =
    "SDKWORK_COMMUNITY_ORDER_SERVICE_TENANT_ID";

/// Overrides the platform service organization id for embedded order
/// management lookups (empty or unset means tenant-level scope).
pub const SDKWORK_COMMUNITY_ORDER_SERVICE_ORGANIZATION_ID_ENV: &str =
    "SDKWORK_COMMUNITY_ORDER_SERVICE_ORGANIZATION_ID";

/// In-process order payment verifier backed by the embedded order repository.
pub struct EmbeddedOrderPaymentVerifier {
    store: PostgresCommerceOrderStore,
    tenant_id: String,
    organization_id: Option<String>,
}

impl EmbeddedOrderPaymentVerifier {
    /// Builds the verifier on the process-shared PostgreSQL pool the
    /// composition root already provides to the embedded assemblies.
    pub fn from_pool(pool: &DatabasePool) -> Result<Self, String> {
        let pg_pool = pool.as_postgres().ok_or_else(|| {
            "embedded order payment verifier requires a PostgreSQL pool".to_owned()
        })?;
        Ok(Self {
            store: PostgresCommerceOrderStore::new(pg_pool.clone()),
            tenant_id: service_tenant_id_from_env(),
            organization_id: service_organization_id_from_env(),
        })
    }
}

impl OrderPaymentVerifier for EmbeddedOrderPaymentVerifier {
    fn verify_order_payment<'a>(&'a self, order_id: &'a str) -> OrderPaymentVerifyFuture<'a> {
        Box::pin(async move {
            let detail = self
                .store
                .retrieve_management_order(OrderManagementDetailQuery {
                    tenant_id: self.tenant_id.clone(),
                    organization_id: self.organization_id.clone(),
                    order_id: order_id.to_owned(),
                })
                .await
                .map_err(|error| {
                    format!(
                        "embedded order payment verification failed: {}",
                        error.message()
                    )
                })?;
            // Unknown order mirrors the HTTP surface: not found is reported
            // as an unpaid verification, not an error.
            let Some(detail) = detail else {
                return Ok(OrderPaymentVerification {
                    paid: false,
                    paid_amount: None,
                });
            };
            let status = detail.summary.status.trim().to_ascii_lowercase();
            let payment_status = detail
                .payment_status
                .as_deref()
                .map(str::trim)
                .map(str::to_ascii_lowercase)
                .unwrap_or_default();
            let paid = is_paid_order_status(&status) || is_paid_payment_status(&payment_status);
            let paid_amount = detail
                .summary
                .paid_amount
                .as_ref()
                .unwrap_or(&detail.summary.total_amount)
                .as_str()
                .parse::<f64>()
                .ok();
            Ok(OrderPaymentVerification { paid, paid_amount })
        })
    }
}

/// Order statuses that indicate a settled, paid order. Mirrors the HTTP
/// adapter's status mapping in `sdkwork-community-service` commerce.rs.
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

/// Payment-record statuses that independently evidence a settled payment.
fn is_paid_payment_status(payment_status: &str) -> bool {
    matches!(payment_status, "paid" | "success" | "payment_success")
}

fn service_tenant_id_from_env() -> String {
    std::env::var(SDKWORK_COMMUNITY_ORDER_SERVICE_TENANT_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty() && value.chars().all(|c| c.is_ascii_digit()))
        .unwrap_or_else(|| "1".to_owned())
}

fn service_organization_id_from_env() -> Option<String> {
    std::env::var(SDKWORK_COMMUNITY_ORDER_SERVICE_ORGANIZATION_ID_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty() && value != "0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_subject_env_parsing() {
        std::env::remove_var(SDKWORK_COMMUNITY_ORDER_SERVICE_TENANT_ID_ENV);
        std::env::remove_var(SDKWORK_COMMUNITY_ORDER_SERVICE_ORGANIZATION_ID_ENV);
        assert_eq!(service_tenant_id_from_env(), "1");
        assert_eq!(service_organization_id_from_env(), None);

        std::env::set_var(SDKWORK_COMMUNITY_ORDER_SERVICE_TENANT_ID_ENV, "7");
        std::env::set_var(SDKWORK_COMMUNITY_ORDER_SERVICE_ORGANIZATION_ID_ENV, "3");
        assert_eq!(service_tenant_id_from_env(), "7");
        assert_eq!(service_organization_id_from_env(), Some("3".to_owned()));

        std::env::set_var(
            SDKWORK_COMMUNITY_ORDER_SERVICE_TENANT_ID_ENV,
            "not-a-number",
        );
        assert_eq!(service_tenant_id_from_env(), "1");
        std::env::set_var(SDKWORK_COMMUNITY_ORDER_SERVICE_ORGANIZATION_ID_ENV, "0");
        assert_eq!(service_organization_id_from_env(), None);
        std::env::remove_var(SDKWORK_COMMUNITY_ORDER_SERVICE_TENANT_ID_ENV);
        std::env::remove_var(SDKWORK_COMMUNITY_ORDER_SERVICE_ORGANIZATION_ID_ENV);
    }

    #[test]
    fn paid_status_mapping_mirrors_http_adapter() {
        assert!(is_paid_order_status("paid"));
        assert!(is_paid_order_status("completed"));
        assert!(!is_paid_order_status("pending"));
        assert!(is_paid_payment_status("paid"));
        assert!(!is_paid_payment_status("refunded"));
    }
}
