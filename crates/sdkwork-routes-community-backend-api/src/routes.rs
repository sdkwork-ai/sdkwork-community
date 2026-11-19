use std::sync::Arc;

use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use sdkwork_community_service::{
    CommunityCategoryCommand, CommunityCircleCommand, CommunityGroupCommand,
    CommunityMemberPatchCommand, CommunityModerationCommand, CommunityService,
    CommunityTierCommand,
};
use sdkwork_community_storage_sqlx::CommunityFeedQuery;
use sdkwork_iam_context_service::IamAppContext;
use sdkwork_routes_community_common::{
    api_response::{map_service_error, success_command, success_item, success_items},
    dto::{map_category, map_entry, map_group, map_member, map_tier},
    subject::runtime_subject_from_extension,
    web_bootstrap::wrap_router_with_web_framework_from_env,
};
use sdkwork_web_core::WebRequestContext;
use serde::Deserialize;

#[derive(Clone)]
struct BackendState {
    service: Arc<CommunityService>,
}

#[derive(Debug, Deserialize)]
struct FeedQueryParams {
    #[serde(alias = "categoryId")]
    category_id: Option<String>,
    kind: Option<String>,
    q: Option<String>,
    #[serde(alias = "reviewState")]
    review_state: Option<String>,
    tag: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct CategoryQueryParams {
    #[serde(alias = "categoryId")]
    category_id: String,
}

#[derive(Debug, Deserialize)]
struct TierQueryParams {
    #[serde(alias = "categoryId")]
    category_id: String,
    #[serde(alias = "enabledOnly")]
    enabled_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct FeatureEntryCommand {
    #[serde(default)]
    featured: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct PinEntryCommand {
    #[serde(default)]
    pinned: Option<bool>,
}

pub fn build_backend_router(
    host: Arc<sdkwork_community_service_host::CommunityServiceHost>,
) -> Router {
    Router::new()
        .route(
            "/backend/v3/api/community/categories",
            get(list_categories).post(create_category),
        )
        .route(
            "/backend/v3/api/community/categories/{categoryId}",
            patch(update_category).delete(delete_category),
        )
        .route(
            "/backend/v3/api/community/circles/{categoryId}",
            patch(update_circle),
        )
        .route("/backend/v3/api/community/entries", get(list_entries))
        .route(
            "/backend/v3/api/community/entries/{entryId}/moderation",
            post(update_moderation),
        )
        .route(
            "/backend/v3/api/community/entries/{entryId}/feature",
            post(feature_entry),
        )
        .route(
            "/backend/v3/api/community/entries/{entryId}/pin",
            post(pin_entry),
        )
        .route(
            "/backend/v3/api/community/entries/{entryId}",
            delete(delete_entry),
        )
        .route(
            "/backend/v3/api/community/moderation/queue",
            get(list_moderation_queue),
        )
        .route(
            "/backend/v3/api/community/recommendations/rebuild",
            post(rebuild_recommendations),
        )
        .route("/backend/v3/api/community/members", get(list_members))
        .route(
            "/backend/v3/api/community/members/{memberId}",
            patch(update_member).delete(remove_member),
        )
        .route(
            "/backend/v3/api/community/groups",
            get(list_groups).post(create_group),
        )
        .route(
            "/backend/v3/api/community/groups/{groupId}",
            patch(update_group).delete(delete_group),
        )
        .route(
            "/backend/v3/api/community/tiers",
            get(list_tiers).post(create_tier),
        )
        .route(
            "/backend/v3/api/community/tiers/{tierId}",
            patch(update_tier).delete(delete_tier),
        )
        .route(
            "/backend/v3/api/community/tiers/{tierId}/publish",
            post(publish_tier),
        )
        .route(
            "/backend/v3/api/community/tiers/{tierId}/unpublish",
            post(unpublish_tier),
        )
        .with_state(BackendState {
            service: host.service(),
        })
}

pub async fn build_backend_router_with_framework(
    host: Arc<sdkwork_community_service_host::CommunityServiceHost>,
) -> Router {
    wrap_router_with_web_framework_from_env(
        build_backend_router(host),
        crate::http_route_manifest::gateway_route_manifest(),
    )
    .await
}

pub async fn gateway_mount(
    host: Arc<sdkwork_community_service_host::CommunityServiceHost>,
) -> Router {
    build_backend_router(host)
}

/// Business-only assembly entrypoint: mounts the community backend router
/// WITHOUT a Web Framework layer. Consuming gateways compose dependency
/// surfaces in-process and install framework/security once on the combined
/// router (API_ASSEMBLY_SPEC §4/§6.1); a nested layer would re-classify the
/// request after the host injected trusted-subject context and reject it as
/// client identity projection (40001).
pub async fn gateway_mount_business(
    host: Arc<sdkwork_community_service_host::CommunityServiceHost>,
) -> Router {
    build_backend_router(host)
}

async fn list_categories(
    State(state): State<BackendState>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state.service.list_categories(&subject.tenant_id).await {
        Ok(items) => {
            let count = items.len() as i64;
            success_items(
                context.as_ref().map(|Extension(ctx)| ctx),
                items.into_iter().map(map_category).collect(),
                1,
                count,
                Some(count),
            )
        }
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn create_category(
    State(state): State<BackendState>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityCategoryCommand>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .create_category(&subject.tenant_id, body)
        .await
    {
        Ok(item) => success_item(
            context.as_ref().map(|Extension(ctx)| ctx),
            map_category(item),
        ),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_category(
    State(state): State<BackendState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityCategoryCommand>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .update_category(&subject.tenant_id, &category_id, body)
        .await
    {
        Ok(item) => success_item(
            context.as_ref().map(|Extension(ctx)| ctx),
            map_category(item),
        ),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn delete_category(
    State(state): State<BackendState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .delete_category(&subject.tenant_id, &category_id)
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_circle(
    State(state): State<BackendState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityCircleCommand>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .update_circle(&subject.tenant_id, &subject.user_id, &category_id, body)
        .await
    {
        Ok(item) => success_item(
            context.as_ref().map(|Extension(ctx)| ctx),
            map_category(item),
        ),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn list_entries(
    State(state): State<BackendState>,
    Query(params): Query<FeedQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let query = CommunityFeedQuery {
        category_id: params.category_id,
        kind: params.kind,
        q: params.q,
        review_state: params.review_state,
        tag: params.tag,
        page: params.page.unwrap_or(1),
        page_size: params.page_size.unwrap_or(20),
        approved_only: false,
    };
    match state.service.list_feed(&subject.tenant_id, query).await {
        Ok(result) => success_items(
            context.as_ref().map(|Extension(ctx)| ctx),
            result.items.into_iter().map(map_entry).collect(),
            result.page,
            result.page_size,
            Some(result.total_items),
        ),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_moderation(
    State(state): State<BackendState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityModerationCommand>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .update_moderation(&subject.tenant_id, &entry_id, &subject.user_id, body)
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_entry(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn feature_entry(
    State(state): State<BackendState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    body: Option<Json<FeatureEntryCommand>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let featured = body.and_then(|command| command.featured).unwrap_or(true);
    match state
        .service
        .set_featured(&subject.tenant_id, &entry_id, featured)
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_entry(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn pin_entry(
    State(state): State<BackendState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    body: Option<Json<PinEntryCommand>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let pinned = body.and_then(|command| command.pinned).unwrap_or(true);
    match state
        .service
        .set_pinned(&subject.tenant_id, &entry_id, pinned)
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_entry(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn delete_entry(
    State(state): State<BackendState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .delete_entry(&subject.tenant_id, &entry_id)
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn list_moderation_queue(
    State(state): State<BackendState>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .list_moderation_queue(&subject.tenant_id)
        .await
    {
        Ok(items) => {
            let count = items.len() as i64;
            success_items(
                context.as_ref().map(|Extension(ctx)| ctx),
                items.into_iter().map(map_entry).collect(),
                1,
                count,
                Some(count),
            )
        }
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn rebuild_recommendations(
    State(state): State<BackendState>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .rebuild_recommendations(&subject.tenant_id)
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn list_members(
    State(state): State<BackendState>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .list_members(&subject.tenant_id, &params.category_id)
        .await
    {
        Ok(items) => {
            let count = items.len() as i64;
            success_items(
                context.as_ref().map(|Extension(ctx)| ctx),
                items.into_iter().map(map_member).collect(),
                1,
                count,
                Some(count),
            )
        }
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_member(
    State(state): State<BackendState>,
    Path(member_id): Path<String>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityMemberPatchCommand>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .update_member(
            &subject.tenant_id,
            &subject.user_id,
            &params.category_id,
            &member_id,
            body,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_member(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn remove_member(
    State(state): State<BackendState>,
    Path(member_id): Path<String>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .remove_member(
            &subject.tenant_id,
            &subject.user_id,
            &params.category_id,
            &member_id,
        )
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn list_groups(
    State(state): State<BackendState>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .list_groups(&subject.tenant_id, &params.category_id)
        .await
    {
        Ok(items) => {
            let count = items.len() as i64;
            success_items(
                context.as_ref().map(|Extension(ctx)| ctx),
                items.into_iter().map(map_group).collect(),
                1,
                count,
                Some(count),
            )
        }
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn create_group(
    State(state): State<BackendState>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityGroupCommand>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .create_group(
            &subject.tenant_id,
            &subject.user_id,
            &params.category_id,
            body,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_group(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_group(
    State(state): State<BackendState>,
    Path(group_id): Path<String>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityGroupCommand>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .update_group(
            &subject.tenant_id,
            &subject.user_id,
            &params.category_id,
            &group_id,
            body,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_group(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn delete_group(
    State(state): State<BackendState>,
    Path(group_id): Path<String>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .delete_group(
            &subject.tenant_id,
            &subject.user_id,
            &params.category_id,
            &group_id,
        )
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn list_tiers(
    State(state): State<BackendState>,
    Query(params): Query<TierQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    let enabled_only = params.enabled_only.unwrap_or(false);
    match state
        .service
        .list_tiers(&subject.tenant_id, &params.category_id, enabled_only)
        .await
    {
        Ok(items) => {
            let count = items.len() as i64;
            success_items(
                context.as_ref().map(|Extension(ctx)| ctx),
                items.into_iter().map(map_tier).collect(),
                1,
                count,
                Some(count),
            )
        }
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn create_tier(
    State(state): State<BackendState>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityTierCommand>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .create_tier(
            &subject.tenant_id,
            &subject.user_id,
            &params.category_id,
            body,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_tier(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_tier(
    State(state): State<BackendState>,
    Path(tier_id): Path<String>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityTierCommand>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .update_tier(
            &subject.tenant_id,
            &subject.user_id,
            &params.category_id,
            &tier_id,
            body,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_tier(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn publish_tier(
    State(state): State<BackendState>,
    Path(tier_id): Path<String>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .publish_tier(
            &subject.tenant_id,
            &subject.user_id,
            &params.category_id,
            &tier_id,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_tier(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn unpublish_tier(
    State(state): State<BackendState>,
    Path(tier_id): Path<String>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .unpublish_tier(
            &subject.tenant_id,
            &subject.user_id,
            &params.category_id,
            &tier_id,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_tier(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn delete_tier(
    State(state): State<BackendState>,
    Path(tier_id): Path<String>,
    Query(params): Query<CategoryQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match auth_subject(context.as_ref().map(|Extension(ctx)| ctx), iam) {
        Ok(subject) => subject,
        Err(response) => return response,
    };
    match state
        .service
        .delete_tier(
            &subject.tenant_id,
            &subject.user_id,
            &params.category_id,
            &tier_id,
        )
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

#[allow(clippy::result_large_err)]
fn auth_subject(
    context: Option<&WebRequestContext>,
    iam: Option<Extension<IamAppContext>>,
) -> Result<sdkwork_routes_community_common::subject::RuntimeSubject, Response> {
    runtime_subject_from_extension(iam).map_err(|error| {
        map_service_error(
            context,
            sdkwork_community_service::CommunityServiceError::Unauthorized(error),
        )
    })
}
