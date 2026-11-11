//! Explicit opt-in HTTP adapters for the commerce integration.
//!
//! `MODULE_SPEC.md` section 4.2 makes in-process the only default: the packaged
//! deployment runs one backend process against one PostgreSQL, so the community
//! module reaches membership and order through their embedded repositories.
//! This module is the deliberate **split deployment** exception: an operator
//! that really runs membership/order as separate services sets the matching
//! `*_BACKEND_API_BASE_URL` plus its dual-token credentials, and only then does
//! the community module speak HTTP to the sibling module.
//!
//! Two guarantees keep that exception from recreating the coupon incident:
//!
//! - the HTTP mode is never a default; a missing base URL means in-process;
//! - [`CommerceIntegrationConfig::from_env`](super::commerce::CommerceIntegrationConfig::from_env)
//!   validates the base URL and both credentials while the host boots, so a
//!   half-configured split deployment fails at startup instead of after a tier
//!   or membership mutation has already been persisted.
//!
//! The request/response shapes are unchanged from the previous adapter: the
//! same DTO bodies, the same dual-token headers, the same envelope unwrapping,
//! and the same status-to-result mapping (`404` on the order lookup means
//! "unpaid", every other rejection is an integration error).

use std::fmt;
use std::time::Duration;

use serde_json::Value;

use super::commerce::{
    is_paid_order_status, MembershipPackageRegistration, OrderPaymentVerification,
    RegisteredMembershipPackage,
};

/// Base URL of a separately deployed membership backend.
pub(crate) const MEMBERSHIP_BASE_URL_ENV: &str = "SDKWORK_MEMBERSHIP_BACKEND_API_BASE_URL";
/// Auth token for the separately deployed membership backend.
pub(crate) const MEMBERSHIP_AUTH_TOKEN_ENV: &str = "SDKWORK_MEMBERSHIP_BACKEND_AUTH_TOKEN";
/// Access token for the separately deployed membership backend.
pub(crate) const MEMBERSHIP_ACCESS_TOKEN_ENV: &str = "SDKWORK_MEMBERSHIP_BACKEND_ACCESS_TOKEN";
/// Base URL of a separately deployed order backend.
pub(crate) const ORDER_BASE_URL_ENV: &str = "SDKWORK_ORDER_BACKEND_API_BASE_URL";
/// Auth token for the separately deployed order backend.
pub(crate) const ORDER_AUTH_TOKEN_ENV: &str = "SDKWORK_ORDER_BACKEND_AUTH_TOKEN";
/// Access token for the separately deployed order backend.
pub(crate) const ORDER_ACCESS_TOKEN_ENV: &str = "SDKWORK_ORDER_BACKEND_ACCESS_TOKEN";

/// Canonical backend credential headers (dual-token; the internal ingress
/// token is used when the backend surface requires internal-api routing).
const ACCESS_TOKEN_HEADER: &str = "Access-Token";
const AUTHORIZATION_HEADER: &str = "Authorization";

/// Bounded waits so a hung split-deployment peer cannot stall a mutation.
const HTTP_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const HTTP_REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

/// A validated split-deployment backend endpoint.
///
/// Constructed only through [`resolve_backend`], which is what makes the
/// "credentials missing" failure a startup failure rather than a request-time
/// one.
#[derive(Clone)]
pub(crate) struct CommerceHttpBackend {
    base_url: String,
    auth_token: String,
    access_token: String,
    http: reqwest::Client,
}

impl fmt::Debug for CommerceHttpBackend {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Endpoint stays diagnosable; credentials never enter logs.
        formatter
            .debug_struct("CommerceHttpBackend")
            .field("base_url", &self.base_url)
            .field("credentials", &"<redacted>")
            .finish()
    }
}

impl CommerceHttpBackend {
    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Ok(value) = reqwest::header::HeaderValue::from_str(&self.access_token) {
            headers.insert(ACCESS_TOKEN_HEADER, value);
        }
        let bearer = format!("Bearer {}", self.auth_token);
        if let Ok(value) = reqwest::header::HeaderValue::from_str(&bearer) {
            headers.insert(AUTHORIZATION_HEADER, value);
        } else if let Ok(value) = reqwest::header::HeaderValue::from_str(&self.auth_token) {
            headers.insert(AUTHORIZATION_HEADER, value);
        }
        headers
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }
}

/// Resolves the membership backend, or `None` for the in-process default.
pub(crate) fn membership_backend_from_env() -> Result<Option<CommerceHttpBackend>, String> {
    resolve_backend(
        "membership",
        env(MEMBERSHIP_BASE_URL_ENV),
        env(MEMBERSHIP_AUTH_TOKEN_ENV),
        env(MEMBERSHIP_ACCESS_TOKEN_ENV),
        MEMBERSHIP_BASE_URL_ENV,
        MEMBERSHIP_AUTH_TOKEN_ENV,
        MEMBERSHIP_ACCESS_TOKEN_ENV,
    )
}

/// Resolves the order backend, or `None` for the in-process default.
pub(crate) fn order_backend_from_env() -> Result<Option<CommerceHttpBackend>, String> {
    resolve_backend(
        "order",
        env(ORDER_BASE_URL_ENV),
        env(ORDER_AUTH_TOKEN_ENV),
        env(ORDER_ACCESS_TOKEN_ENV),
        ORDER_BASE_URL_ENV,
        ORDER_AUTH_TOKEN_ENV,
        ORDER_ACCESS_TOKEN_ENV,
    )
}

/// Pure transport resolution: an unset/blank base URL selects in-process, and a
/// base URL without both credentials is a startup error (never a fallback).
pub(crate) fn resolve_backend(
    module: &str,
    base_url: Option<String>,
    auth_token: Option<String>,
    access_token: Option<String>,
    base_url_env: &str,
    auth_token_env: &str,
    access_token_env: &str,
) -> Result<Option<CommerceHttpBackend>, String> {
    let Some(base_url) = trimmed(base_url) else {
        // In-process default. Stray credentials without an opt-in URL are
        // ignored: they cannot turn HTTP on by themselves.
        return Ok(None);
    };
    let base_url = base_url.trim_end_matches('/').to_owned();
    let auth_token = trimmed(auth_token).ok_or_else(|| {
        format!(
            "{base_url_env} selects the split-deployment HTTP {module} backend, but \
             {auth_token_env} is missing; set both backend credentials or unset \
             {base_url_env} to use the in-process {module} port"
        )
    })?;
    let access_token = trimmed(access_token).ok_or_else(|| {
        format!(
            "{base_url_env} selects the split-deployment HTTP {module} backend, but \
             {access_token_env} is missing; set both backend credentials or unset \
             {base_url_env} to use the in-process {module} port"
        )
    })?;
    let http = build_http_client()?;
    Ok(Some(CommerceHttpBackend {
        base_url,
        auth_token,
        access_token,
        http,
    }))
}

fn build_http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .connect_timeout(HTTP_CONNECT_TIMEOUT)
        .timeout(HTTP_REQUEST_TIMEOUT)
        .build()
        .map_err(|error| format!("split-deployment commerce HTTP client failed: {error}"))
}

fn env(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

fn trimmed(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

/// Registers a purchasable membership package through the split-deployment
/// membership backend and returns the assigned external id.
pub(crate) async fn register_membership_package(
    backend: &CommerceHttpBackend,
    registration: MembershipPackageRegistration,
) -> Result<RegisteredMembershipPackage, String> {
    let url = backend.url("/backend/v3/api/memberships/packages");
    let response = backend
        .http
        .post(&url)
        .headers(backend.headers())
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
    let item = envelope_item(&payload);
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
        .ok_or_else(|| "membership package registration did not return externalId".to_owned())?;
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

/// Verifies an order's payment state through the split-deployment order
/// backend. A `404` is reported as "not paid" exactly like the in-process
/// adapter's missing row; every other rejection maps to an integration error.
pub(crate) async fn verify_order_payment(
    backend: &CommerceHttpBackend,
    order_id: &str,
) -> Result<OrderPaymentVerification, String> {
    let url = backend.url(&format!("/backend/v3/api/orders/{}", urlencode(order_id)));
    let response = backend
        .http
        .get(&url)
        .headers(backend.headers())
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
    let item = envelope_item(&payload);
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

fn envelope_item(payload: &Value) -> &Value {
    payload
        .get("data")
        .and_then(|data| data.get("item"))
        .or_else(|| payload.get("data"))
        .unwrap_or(payload)
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

#[cfg(test)]
mod tests {
    use super::*;

    const MEMBERSHIP_BASE: &str = "SDKWORK_MEMBERSHIP_BACKEND_API_BASE_URL";
    const MEMBERSHIP_AUTH: &str = "SDKWORK_MEMBERSHIP_BACKEND_AUTH_TOKEN";
    const MEMBERSHIP_ACCESS: &str = "SDKWORK_MEMBERSHIP_BACKEND_ACCESS_TOKEN";

    fn resolve(
        base: Option<&str>,
        auth: Option<&str>,
        access: Option<&str>,
    ) -> Result<Option<CommerceHttpBackend>, String> {
        resolve_backend(
            "membership",
            base.map(str::to_owned),
            auth.map(str::to_owned),
            access.map(str::to_owned),
            MEMBERSHIP_BASE,
            MEMBERSHIP_AUTH,
            MEMBERSHIP_ACCESS,
        )
    }

    #[test]
    fn an_unset_base_url_keeps_the_in_process_default() {
        assert!(resolve(None, None, None)
            .expect("in-process default")
            .is_none());
        // Blank values are unset, not an endpoint.
        assert!(resolve(Some("   "), None, None)
            .expect("blank base url is unset")
            .is_none());
    }

    #[test]
    fn credentials_alone_cannot_turn_http_on() {
        assert!(resolve(None, Some("auth"), Some("access"))
            .expect("credentials without a base url stay in-process")
            .is_none());
    }

    #[test]
    fn a_base_url_without_credentials_fails_at_startup() {
        let error = resolve(Some("http://membership.internal"), None, Some("access"))
            .expect_err("missing auth token must fail startup");
        assert!(error.contains(MEMBERSHIP_BASE), "{error}");
        assert!(error.contains(MEMBERSHIP_AUTH), "{error}");
        assert!(error.contains("in-process"), "{error}");

        let error = resolve(Some("http://membership.internal"), Some("auth"), None)
            .expect_err("missing access token must fail startup");
        assert!(error.contains(MEMBERSHIP_ACCESS), "{error}");
    }

    #[test]
    fn a_fully_configured_base_url_selects_http() {
        let backend = resolve(
            Some(" http://membership.internal/ "),
            Some(" auth "),
            Some(" access "),
        )
        .expect("validated split deployment")
        .expect("http backend");
        assert_eq!(backend.base_url, "http://membership.internal");
        assert_eq!(backend.auth_token, "auth");
        assert_eq!(backend.access_token, "access");
        assert_eq!(
            backend.url("/backend/v3/api/memberships/packages"),
            "http://membership.internal/backend/v3/api/memberships/packages"
        );
    }

    #[test]
    fn order_backend_resolution_uses_the_order_variables() {
        let error = resolve_backend(
            "order",
            Some("http://order.internal".to_owned()),
            None,
            None,
            ORDER_BASE_URL_ENV,
            ORDER_AUTH_TOKEN_ENV,
            ORDER_ACCESS_TOKEN_ENV,
        )
        .expect_err("split order deployment without credentials must fail");
        assert!(error.contains(ORDER_BASE_URL_ENV), "{error}");
        assert!(error.contains(ORDER_AUTH_TOKEN_ENV), "{error}");
    }
}
