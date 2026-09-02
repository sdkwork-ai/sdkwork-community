//! Gateway bootstrap for sdkwork-community.

use axum::Router;
use sdkwork_community_service_host::CommunityServiceHost;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_web_bootstrap::{ApiAssemblyContribution, DatabasePoolReadinessCheck, WebModule};
use sdkwork_web_core::HttpRouteManifest;
use serde_json::Value;
use std::sync::Arc;

pub type ApiAssembly = ApiAssemblyContribution;

/// Community App API route manifest (App surface only).
///
/// Host gateways that merge the unwrapped App surface contribution compose
/// this manifest into their own surface route inventory so the Web Framework
/// honors the App routes' declared authentication and permissions
/// (API_ASSEMBLY_SPEC §3 — the host owns the single framework layer and
/// composes capability manifests; it never re-declares capability routes).
pub fn app_api_route_manifest() -> HttpRouteManifest {
    sdkwork_routes_community_app_api::gateway_route_manifest()
}

/// Host-neutral Community API contribution plus its process-shared database pool.
pub struct CommunityApiRuntime {
    pub contribution: ApiAssembly,
    pub database_pool: DatabasePool,
}

pub async fn assemble_api_router_runtime() -> Result<CommunityApiRuntime, String> {
    let host = Arc::new(CommunityServiceHost::from_env().await?);
    let database_pool = host.database_pool().clone();
    let contribution = assemble_api_router_with_host(host)?;
    Ok(CommunityApiRuntime {
        contribution,
        database_pool,
    })
}

pub async fn assemble_api_router() -> Result<ApiAssembly, String> {
    Ok(assemble_api_router_runtime().await?.contribution)
}

/// Builds the community service host on a process-shared pool. With the
/// `commerce-membership-embedded` / `commerce-order-embedded` features
/// selected, the composition wires the in-process commerce ports (membership
/// package publisher and order payment verifier) so commerce consumes the
/// embedded dependency capabilities directly (`APPLICATION_GATEWAY_SPEC.md`
/// section 2.3) instead of looping HTTP requests through the host's own
/// listener.
pub async fn host_from_pool(pool: DatabasePool) -> Result<Arc<CommunityServiceHost>, String> {
    #[cfg(feature = "commerce-membership-embedded")]
    let membership_publisher = {
        Some(Arc::new(
            sdkwork_community_commerce_membership_embedded::EmbeddedMembershipPackagePublisher::from_pool(&pool)
                .map_err(|error| {
                    format!("embedded membership commerce publisher unavailable: {error}")
                })?,
        ) as Arc<dyn sdkwork_community_service_host::MembershipPackagePublisher>)
    };
    #[cfg(not(feature = "commerce-membership-embedded"))]
    let membership_publisher = None;
    #[cfg(feature = "commerce-order-embedded")]
    let order_verifier = {
        Some(Arc::new(
            sdkwork_community_commerce_order_embedded::EmbeddedOrderPaymentVerifier::from_pool(
                &pool,
            )
            .map_err(|error| format!("embedded order commerce verifier unavailable: {error}"))?,
        )
            as Arc<
                dyn sdkwork_community_service_host::OrderPaymentVerifier,
            >)
    };
    #[cfg(not(feature = "commerce-order-embedded"))]
    let order_verifier = None;
    CommunityServiceHost::from_database_pool_with_commerce_ports(
        pool,
        membership_publisher,
        order_verifier,
    )
    .await
}

pub async fn assemble_api_router_with_pool(pool: DatabasePool) -> Result<ApiAssembly, String> {
    let host = host_from_pool(pool).await?;
    assemble_api_router_with_host(host)
}

/// Community backend business router for consuming hosts
/// (API_ASSEMBLY_SPEC §3 federated backend entrypoint).
pub struct BusinessRouterAssembly {
    pub router: Router,
}

/// Compose the Community backend business router on a host-neutral service
/// host. Mirrors the membership/payment backend business assemblies: the
/// consuming gateway merges the router before installing its own Web
/// Framework layer.
pub async fn assemble_backend_business_router(
    host: Arc<CommunityServiceHost>,
) -> BusinessRouterAssembly {
    BusinessRouterAssembly {
        router: sdkwork_routes_community_backend_api::gateway_mount_business(host).await,
    }
}

pub async fn assemble_backend_business_router_from_env() -> Result<BusinessRouterAssembly, String> {
    let host = Arc::new(CommunityServiceHost::from_env().await?);
    Ok(assemble_backend_business_router(host).await)
}

/// Same as [`assemble_backend_business_router_from_env`] but from an
/// already-shared process database pool (platform gateways with a single
/// shared pool; API_ASSEMBLY_SPEC §6.1 same-origin dependency composition).
pub async fn assemble_backend_business_router_with_pool(
    pool: DatabasePool,
) -> Result<BusinessRouterAssembly, String> {
    let host = host_from_pool(pool).await?;
    Ok(assemble_backend_business_router(host).await)
}

/// Builds the unwrapped Community App API for a gateway that owns the single
/// Web Framework layer (API_ASSEMBLY_SPEC §3 federated contribution entrypoint).
///
/// The App surface (`/app/v3/api/community/*`) is composed from the
/// environment database profile so the host gateway reuses its own IAM/web
/// framework wiring; handlers resolve `IamAppContext` from the host-injected
/// domain context and do not install a framework of their own.
pub async fn assemble_app_api_contribution() -> Result<ApiAssembly, String> {
    let host = Arc::new(CommunityServiceHost::from_env().await?);
    assemble_app_api_contribution_with_host(host)
}

/// Same as [`assemble_app_api_contribution`] but from an already-shared
/// process database pool (platform gateways with a single shared pool).
pub async fn assemble_app_api_contribution_with_pool(
    pool: DatabasePool,
) -> Result<ApiAssembly, String> {
    let host = host_from_pool(pool).await?;
    assemble_app_api_contribution_with_host(host)
}

/// Composes the App surface from an already-created host so the caller can
/// reuse the same host (and pool) across surfaces — e.g. the IM standalone
/// gateway mounts both the App API and the open surface (feeds source
/// adapter data source) from one `CommunityServiceHost`.
pub fn assemble_app_api_contribution_with_host(
    host: Arc<CommunityServiceHost>,
) -> Result<ApiAssembly, String> {
    let database_pool = host.database_pool().clone();
    let router = sdkwork_routes_community_app_api::build_app_router(host);
    ApiAssemblyContribution::from_manifest(
        "sdkwork-community",
        "SDKWork Community App API",
        router,
        sdkwork_routes_community_app_api::gateway_route_manifest(),
        Vec::new(),
        Arc::new(DatabasePoolReadinessCheck::new(database_pool)),
    )
}

/// Composes the public open surface (`/community/v3/api/*`) from an
/// already-created host. Anonymous read surfaces such as `feed.public.list`
/// are the data source for the feeds community adapter.
pub fn assemble_open_api_contribution_with_host(
    host: Arc<CommunityServiceHost>,
) -> Result<ApiAssembly, String> {
    let database_pool = host.database_pool().clone();
    let router = sdkwork_routes_community_open_api::build_open_router(host);
    ApiAssemblyContribution::from_manifest(
        "sdkwork-community-open",
        "SDKWork Community Open API",
        router,
        sdkwork_routes_community_open_api::gateway_route_manifest(),
        Vec::new(),
        Arc::new(DatabasePoolReadinessCheck::new(database_pool)),
    )
}

pub fn assemble_api_router_with_host(
    host: Arc<CommunityServiceHost>,
) -> Result<ApiAssembly, String> {
    let database_pool = host.database_pool().clone();
    let router = Router::new()
        .merge(sdkwork_routes_community_app_api::build_app_router(
            host.clone(),
        ))
        .merge(sdkwork_routes_community_backend_api::build_backend_router(
            host.clone(),
        ))
        .merge(sdkwork_routes_community_open_api::build_open_router(host));
    let route_manifest = combined_route_manifest();
    ApiAssemblyContribution::from_openapi_documents(
        "sdkwork-community",
        "SDKWork Community API",
        router,
        route_manifest,
        authored_openapi_documents()?,
        Vec::new(),
        Arc::new(DatabasePoolReadinessCheck::new(database_pool)),
    )
}

fn combined_route_manifest() -> HttpRouteManifest {
    let manifests = [
        sdkwork_routes_community_app_api::gateway_route_manifest(),
        sdkwork_routes_community_backend_api::gateway_route_manifest(),
        sdkwork_routes_community_open_api::gateway_route_manifest(),
    ];
    HttpRouteManifest::from_owned_routes(
        manifests
            .into_iter()
            .flat_map(|manifest| manifest.routes().to_vec())
            .collect(),
    )
}

fn authored_openapi_documents() -> Result<Vec<Value>, String> {
    [
        include_str!("../../../apis/app-api/community/openapi.json"),
        include_str!("../../../apis/backend-api/community/openapi.json"),
        include_str!("../../../apis/open-api/community/openapi.json"),
    ]
    .into_iter()
    .map(|document| {
        serde_json::from_str(document)
            .map_err(|error| format!("parse Community OpenAPI document failed: {error}"))
    })
    .collect()
}

/// Canonical Web Module definition for this application
/// (API_ASSEMBLY_SPEC §4.1.1): the complete HTTP surface — every route,
/// manifest, and OpenAPI document of this owner — as one installable module.
pub async fn web_module() -> Result<WebModule, String> {
    Ok(WebModule::from_contribution(assemble_api_router().await?))
}

/// Same as [`web_module`] but composed on a process-shared database pool
/// (platform gateways, API_ASSEMBLY_SPEC §4.1.1).
pub async fn web_module_with_pool(pool: DatabasePool) -> Result<WebModule, String> {
    Ok(WebModule::from_contribution(
        assemble_api_router_with_pool(pool).await?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_web_core::HttpMethod;

    #[test]
    fn app_api_contribution_uses_app_surface_entrypoints() {
        let source = include_str!("bootstrap.rs");

        assert!(source.contains("pub async fn assemble_app_api_contribution("));
        assert!(source.contains("sdkwork_routes_community_app_api::build_app_router("));
        assert!(source.contains("sdkwork_routes_community_app_api::gateway_route_manifest()"));
        assert!(source.contains("ApiAssemblyContribution::from_manifest("));
        // The app-surface entrypoint body must not mount the backend/open
        // surfaces (those belong to the host-neutral all-surface assembly).
        let assembly_index = source
            .find("pub async fn assemble_app_api_contribution(")
            .expect("app contribution entrypoint");
        let body_end = source[assembly_index..]
            .find("\npub fn assemble_open_api_contribution_with_host")
            .map(|offset| assembly_index + offset)
            .unwrap_or(source.len());
        let assembled = &source[assembly_index..body_end];
        for forbidden in [
            "sdkwork_routes_community_backend_api::build_backend_router",
            "sdkwork_routes_community_open_api::build_open_router",
        ] {
            assert!(
                !assembled.contains(forbidden),
                "app contribution must not mount {forbidden}"
            );
        }
    }

    #[test]
    fn backend_business_router_uses_community_backend_entrypoint() {
        let source = include_str!("bootstrap.rs");

        assert!(source.contains("pub struct BusinessRouterAssembly"));
        assert!(source.contains("pub async fn assemble_backend_business_router("));
        assert!(source.contains("sdkwork_routes_community_backend_api::gateway_mount_business("));
        assert!(source.contains("pub async fn assemble_backend_business_router_with_pool("));
        // The backend entrypoint must not mount the app/open surfaces (those
        // belong to the host-neutral all-surface assembly).
        let backend_index = source
            .find("pub async fn assemble_backend_business_router(")
            .expect("backend business router entrypoint");
        let body_end = source[backend_index..]
            .find("\npub async fn assemble_backend_business_router_from_env()")
            .map(|offset| backend_index + offset)
            .unwrap_or(source.len());
        let assembled = &source[backend_index..body_end];
        for forbidden in [
            "sdkwork_routes_community_app_api::build_app_router",
            "sdkwork_routes_community_open_api::build_open_router",
        ] {
            assert!(
                !assembled.contains(forbidden),
                "backend business router must not mount {forbidden}"
            );
        }
    }

    #[test]
    fn authored_openapi_matches_typed_route_permissions() {
        let documents = authored_openapi_documents().expect("authored OpenAPI documents");
        let manifest = combined_route_manifest();

        for route in manifest.routes() {
            let method = method_label(route.method);
            let operation = documents
                .iter()
                .find_map(|document| document["paths"][route.path][method].as_object())
                .unwrap_or_else(|| panic!("missing authored operation {method} {}", route.path));

            assert_eq!(
                operation.get("operationId").and_then(Value::as_str),
                Some(route.operation_id),
                "operation id drift for {method} {}",
                route.path,
            );
            assert_eq!(
                operation
                    .get("x-sdkwork-permission")
                    .and_then(Value::as_str),
                route.required_permission,
                "permission drift for {method} {}",
                route.path,
            );
        }
    }

    fn method_label(method: HttpMethod) -> &'static str {
        match method {
            HttpMethod::Delete => "delete",
            HttpMethod::Get => "get",
            HttpMethod::Patch => "patch",
            HttpMethod::Post => "post",
            HttpMethod::Put => "put",
        }
    }
}
