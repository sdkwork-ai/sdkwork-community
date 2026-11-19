use std::collections::BTreeMap;
use std::sync::Arc;

use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use sdkwork_community_service::{
    CommunityActivateMembershipCommand, CommunityCircleCommand, CommunityCommentCommand,
    CommunityEntryCommand, CommunityGroupCommand, CommunityMemberPatchCommand,
    CommunityReactionCommand, CommunityService, CommunityTierCommand,
};
use sdkwork_community_storage_sqlx::CommunityFeedQuery;
use sdkwork_iam_context_service::IamAppContext;
use sdkwork_routes_community_common::{
    api_response::{map_service_error, success_command, success_item, success_items},
    dto::{map_category, map_comment, map_entry, map_group, map_member, map_tier},
    subject::{runtime_subject_from_extension, runtime_subject_from_extension_or_public},
    web_bootstrap::wrap_router_with_web_framework_from_env,
};
use sdkwork_web_core::WebRequestContext;
use serde::Deserialize;

#[derive(Clone)]
struct AppState {
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

pub fn build_app_router(host: Arc<sdkwork_community_service_host::CommunityServiceHost>) -> Router {
    Router::new()
        .route(
            "/app/v3/api/community/categories",
            get(list_categories).post(create_circle),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}",
            get(retrieve_circle)
                .patch(update_circle)
                .delete(delete_circle),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/join",
            post(join_category),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/members",
            get(list_members),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/members/activate",
            post(activate_membership),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/members/current",
            get(current_member),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/members/{memberId}",
            patch(update_member).delete(remove_member),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/groups",
            get(list_groups).post(create_group),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/groups/{groupId}",
            patch(update_group).delete(delete_group),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/tiers",
            get(list_tiers).post(create_tier),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/tiers/{tierId}",
            patch(update_tier).delete(delete_tier),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/tiers/{tierId}/publish",
            post(publish_tier),
        )
        .route(
            "/app/v3/api/community/categories/{categoryId}/tiers/{tierId}/unpublish",
            post(unpublish_tier),
        )
        .route("/app/v3/api/community/feed", get(list_feed))
        .route("/app/v3/api/community/entries", post(create_entry))
        .route(
            "/app/v3/api/community/entries/{entryId}",
            get(retrieve_entry).patch(update_entry).delete(delete_entry),
        )
        .route(
            "/app/v3/api/community/entries/{entryId}/reactions",
            post(set_reaction),
        )
        .route(
            "/app/v3/api/community/entries/{entryId}/recommendations",
            get(list_recommendations),
        )
        .route(
            "/app/v3/api/community/entries/{entryId}/publication_readiness",
            get(publication_readiness),
        )
        .route(
            "/app/v3/api/community/entries/{entryId}/comments",
            get(list_comments).post(create_comment),
        )
        .with_state(AppState {
            service: host.service(),
        })
}

pub async fn build_app_router_with_framework(
    host: Arc<sdkwork_community_service_host::CommunityServiceHost>,
) -> Router {
    wrap_router_with_web_framework_from_env(
        build_app_router(host),
        crate::http_route_manifest::gateway_route_manifest(),
    )
    .await
}

pub async fn gateway_mount(
    host: Arc<sdkwork_community_service_host::CommunityServiceHost>,
) -> Router {
    build_app_router(host)
}

async fn list_categories(
    State(state): State<AppState>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .list_categories_with_membership(&subject.tenant_id, &subject.user_id)
        .await
    {
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

async fn list_feed(
    State(state): State<AppState>,
    Query(params): Query<FeedQueryParams>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    // `feed.list` is a public read surface: authenticated callers keep their
    // own tenant scoping, anonymous callers read the public default tenant.
    let subject = match runtime_subject_from_extension_or_public(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Validation(error),
            )
        }
    };
    let query = CommunityFeedQuery {
        category_id: params.category_id,
        kind: params.kind,
        q: params.q,
        review_state: params.review_state,
        tag: params.tag,
        page: params.page.unwrap_or(1),
        page_size: params.page_size.unwrap_or(20),
        approved_only: true,
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

async fn create_entry(
    State(state): State<AppState>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityEntryCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .create_entry(
            &subject.tenant_id,
            &subject.user_id,
            &subject.display_name,
            body,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_entry(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn retrieve_entry(
    State(state): State<AppState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .retrieve_entry(&subject.tenant_id, &entry_id, false)
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_entry(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_entry(
    State(state): State<AppState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityEntryCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .update_entry(&subject.tenant_id, &entry_id, body)
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_entry(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn list_recommendations(
    State(state): State<AppState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .list_recommendations(&subject.tenant_id, &entry_id)
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

async fn publication_readiness(
    State(state): State<AppState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .publication_readiness(&subject.tenant_id, &entry_id)
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn list_comments(
    State(state): State<AppState>,
    Path(entry_id): Path<String>,
    Query(query): Query<BTreeMap<String, String>>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    let page = query
        .get("page")
        .and_then(|value| value.parse::<i64>().ok())
        .filter(|value| *value >= 1)
        .unwrap_or(1);
    let page_size = query
        .get("page_size")
        .and_then(|value| value.parse::<i64>().ok())
        .filter(|value| (1..=200).contains(value))
        .unwrap_or(20);
    match state
        .service
        .list_comments(&subject.tenant_id, &entry_id, page, page_size)
        .await
    {
        Ok((items, total)) => success_items(
            context.as_ref().map(|Extension(ctx)| ctx),
            items.into_iter().map(map_comment).collect(),
            page,
            page_size,
            Some(total),
        ),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn create_comment(
    State(state): State<AppState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityCommentCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .create_comment(
            &subject.tenant_id,
            &entry_id,
            &subject.user_id,
            &subject.display_name,
            body,
        )
        .await
    {
        Ok(item) => success_item(
            context.as_ref().map(|Extension(ctx)| ctx),
            map_comment(item),
        ),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn set_reaction(
    State(state): State<AppState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityReactionCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .set_reaction(&subject.tenant_id, &entry_id, &subject.user_id, body)
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn delete_entry(
    State(state): State<AppState>,
    Path(entry_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .delete_entry_for_author(&subject.tenant_id, &subject.user_id, &entry_id)
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn create_circle(
    State(state): State<AppState>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityCircleCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .create_circle(
            &subject.tenant_id,
            &subject.user_id,
            &subject.display_name,
            body,
        )
        .await
    {
        Ok(item) => success_item(
            context.as_ref().map(|Extension(ctx)| ctx),
            map_category(item),
        ),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_circle(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityCircleCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
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

async fn retrieve_circle(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .retrieve_category_with_membership(&subject.tenant_id, &category_id, &subject.user_id)
        .await
    {
        Ok(item) => success_item(
            context.as_ref().map(|Extension(ctx)| ctx),
            map_category(item),
        ),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn delete_circle(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .delete_circle(&subject.tenant_id, &subject.user_id, &category_id)
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn join_category(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .join_category(
            &subject.tenant_id,
            &category_id,
            &subject.user_id,
            &subject.display_name,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_member(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn list_members(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .list_members(&subject.tenant_id, &category_id)
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

async fn current_member(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .current_member(&subject.tenant_id, &category_id, &subject.user_id)
        .await
    {
        Ok(Some(member)) => success_item(
            context.as_ref().map(|Extension(ctx)| ctx),
            map_member(member),
        ),
        // A `/members/current` lookup for a user who has not joined is an
        // absence, not an error: 200 with `data.item: null` so clients can
        // distinguish "not a member" from a failed lookup without treating
        // 404 as an exception.
        Ok(None) => success_item(
            context.as_ref().map(|Extension(ctx)| ctx),
            Option::<sdkwork_routes_community_common::dto::CommunityMemberResponse>::None,
        ),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_member(
    State(state): State<AppState>,
    Path((category_id, member_id)): Path<(String, String)>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityMemberPatchCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .update_member(
            &subject.tenant_id,
            &subject.user_id,
            &category_id,
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
    State(state): State<AppState>,
    Path((category_id, member_id)): Path<(String, String)>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .remove_member(
            &subject.tenant_id,
            &subject.user_id,
            &category_id,
            &member_id,
        )
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn list_groups(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .list_groups(&subject.tenant_id, &category_id)
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
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityGroupCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .create_group(&subject.tenant_id, &subject.user_id, &category_id, body)
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_group(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_group(
    State(state): State<AppState>,
    Path((category_id, group_id)): Path<(String, String)>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityGroupCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .update_group(
            &subject.tenant_id,
            &subject.user_id,
            &category_id,
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
    State(state): State<AppState>,
    Path((category_id, group_id)): Path<(String, String)>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .delete_group(
            &subject.tenant_id,
            &subject.user_id,
            &category_id,
            &group_id,
        )
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn list_tiers(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    Query(query): Query<BTreeMap<String, String>>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    // Owners see the full list including unpublished tiers; the purchase
    // surface only receives enabled tiers.
    let include_disabled = query.get("includeDisabled").is_some();
    match state
        .service
        .list_tiers(&subject.tenant_id, &category_id, !include_disabled)
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
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityTierCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .create_tier(&subject.tenant_id, &subject.user_id, &category_id, body)
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_tier(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn update_tier(
    State(state): State<AppState>,
    Path((category_id, tier_id)): Path<(String, String)>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityTierCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .update_tier(
            &subject.tenant_id,
            &subject.user_id,
            &category_id,
            &tier_id,
            body,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_tier(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn delete_tier(
    State(state): State<AppState>,
    Path((category_id, tier_id)): Path<(String, String)>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .delete_tier(&subject.tenant_id, &subject.user_id, &category_id, &tier_id)
        .await
    {
        Ok(item) => success_command(context.as_ref().map(|Extension(ctx)| ctx), item),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn publish_tier(
    State(state): State<AppState>,
    Path((category_id, tier_id)): Path<(String, String)>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .publish_tier(&subject.tenant_id, &subject.user_id, &category_id, &tier_id)
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_tier(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn unpublish_tier(
    State(state): State<AppState>,
    Path((category_id, tier_id)): Path<(String, String)>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .unpublish_tier(&subject.tenant_id, &subject.user_id, &category_id, &tier_id)
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_tier(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

async fn activate_membership(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    context: Option<Extension<WebRequestContext>>,
    iam: Option<Extension<IamAppContext>>,
    Json(body): Json<CommunityActivateMembershipCommand>,
) -> Response {
    let subject = match runtime_subject_from_extension(iam) {
        Ok(subject) => subject,
        Err(error) => {
            return map_service_error(
                context.as_ref().map(|Extension(ctx)| ctx),
                sdkwork_community_service::CommunityServiceError::Unauthorized(error),
            )
        }
    };
    match state
        .service
        .activate_membership(
            &subject.tenant_id,
            &category_id,
            &subject.user_id,
            &subject.display_name,
            body,
        )
        .await
    {
        Ok(item) => success_item(context.as_ref().map(|Extension(ctx)| ctx), map_member(item)),
        Err(error) => map_service_error(context.as_ref().map(|Extension(ctx)| ctx), error),
    }
}

#[cfg(test)]
mod query_contract_tests {
    use super::FeedQueryParams;

    #[test]
    fn feed_query_accepts_canonical_snake_case() {
        let query: FeedQueryParams = serde_json::from_value(serde_json::json!({
            "category_id": "1001",
            "review_state": "pending-review"
        }))
        .expect("canonical query names");
        assert_eq!(query.category_id.as_deref(), Some("1001"));
        assert_eq!(query.review_state.as_deref(), Some("pending-review"));
    }

    #[test]
    fn feed_query_keeps_legacy_camel_case_aliases() {
        let query: FeedQueryParams = serde_json::from_value(serde_json::json!({
            "categoryId": "1001",
            "reviewState": "approved"
        }))
        .expect("legacy query names");
        assert_eq!(query.category_id.as_deref(), Some("1001"));
        assert_eq!(query.review_state.as_deref(), Some("approved"));
    }
}
