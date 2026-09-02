//! In-process community source adapter for feeds (`source_type = community.entry`).
//!
//! This adapter is the embedded-deployment counterpart of
//! `sdkwork-feeds-source-community`. Both project the same standardized stream
//! items, but this one reads the community domain through `CommunityService`
//! directly, so a collapsed single-ingress gateway never issues an HTTP request
//! back into its own listener (APPLICATION_GATEWAY_SPEC §2.3).
//!
//! The projection contract is identical to the HTTP adapter: only whitelisted
//! source fields are carried into the stream payload, raw source documents
//! never land in the projection.

use std::sync::Arc;

use async_trait::async_trait;
use sdkwork_community_service::CommunityService;
// Both dependency packages declare an explicit `[lib] name` without the
// `-rust` package suffix, so the crate identifiers differ from the package
// names: `sdkwork-community-storage-sqlx-rust` -> `sdkwork_community_storage_sqlx`.
use sdkwork_community_storage_sqlx::CommunityFeedQuery;
use sdkwork_content_feeds_core::FeedSourceItem;
use sdkwork_content_feeds_service::{FeedSourceAdapter, FeedsServiceError, FeedsServiceResult};

const SOURCE_TYPE: &str = "community.entry";
/// Upper bound per service call; `CommunityService` caps `page_size` at 200.
const MAX_PAGE_SIZE: i64 = 100;
/// Fallback timestamp for entries without an explicit publication time.
const DEFAULT_PUBLISHED_AT: &str = "1970-01-01T00:00:00Z";

/// Whitelisted inspiration payload fields mapped by the adapter. Raw source
/// documents never land in the stream projection; only these standardized
/// fields are carried for stream rendering (frontends read `payload` instead
/// of parsing arbitrary source bodies).
const PAYLOAD_WHITELIST: &[&str] = &[
    "src",
    "alt",
    "prompt",
    "avatar",
    "videoUrl",
    "cover",
    "duration",
    "desc",
    "banner",
    "works",
    "status",
    "tag",
    "participants",
    "background",
    "timeRange",
    "aspectRatio",
    "model",
    "isBanner",
    "kind",
];

/// Stream key -> community feed query filters: `(category_id, kind, tag)`.
fn query_filters(stream_key: &str) -> (Option<String>, Option<String>, Option<String>) {
    if let Some(rest) = stream_key.strip_prefix("community-") {
        if let Some(circle_id) = rest.strip_suffix("-resources") {
            // Circle resources stream: kind=resource only.
            return (
                Some(circle_id.to_owned()),
                Some("resource".to_owned()),
                None,
            );
        }
        return (Some(rest.to_owned()), None, None);
    }
    if let Some(rest) = stream_key.strip_prefix("moments-") {
        if rest == "global" {
            return (None, Some("discussion".to_owned()), None);
        }
        return (Some(rest.to_owned()), Some("discussion".to_owned()), None);
    }
    if let Some(tag) = stream_key.strip_prefix("agents-inspiration-") {
        return (None, None, Some(tag.to_owned()));
    }
    (None, None, None)
}

/// Maps the source JSON body onto the whitelisted standardized payload.
fn payload_from_body(kind: &str, body: Option<&str>) -> Option<String> {
    let mut mapped = serde_json::Map::new();
    mapped.insert(
        "kind".to_owned(),
        serde_json::Value::String(kind.to_owned()),
    );
    if let Some(raw) = body {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(raw) {
            if let Some(object) = parsed.as_object() {
                for key in PAYLOAD_WHITELIST {
                    if *key == "kind" {
                        continue;
                    }
                    if let Some(value) = object.get(*key) {
                        mapped.insert((*key).to_owned(), value.clone());
                    }
                }
            }
        }
    }
    if mapped.len() == 1 && kind.is_empty() {
        return None;
    }
    Some(serde_json::Value::Object(mapped).to_string())
}

/// Extracts a cover image URL from the standardized payload.
fn cover_from_payload(payload: Option<&str>) -> Option<String> {
    let parsed = serde_json::from_str::<serde_json::Value>(payload?).ok()?;
    let object = parsed.as_object()?;
    object
        .get("src")
        .or_else(|| object.get("cover"))
        .and_then(|value| value.as_str())
        .map(str::to_owned)
        .filter(|value| !value.is_empty())
}

/// Projects a community entry into the standardized feeds stream item.
fn project(entry: sdkwork_community_service::CommunityEntryView) -> FeedSourceItem {
    let payload = payload_from_body(&entry.kind, entry.body.as_deref());
    let cover_url = cover_from_payload(payload.as_deref());
    FeedSourceItem {
        source_id: entry.id,
        title: entry.title,
        excerpt: entry.excerpt,
        payload,
        cover_url,
        author_id: Some(entry.author_id).filter(|value| !value.is_empty()),
        author_name: Some(entry.author_name).filter(|value| !value.is_empty()),
        author_avatar_url: None,
        rank_score: 0.0,
        reaction_count: entry.reaction_count,
        comment_count: entry.comment_count,
        published_at: entry
            .published_at
            .or(entry.last_activity_at)
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| DEFAULT_PUBLISHED_AT.to_owned()),
    }
}

/// Community source adapter backed by an in-process `CommunityService`.
pub struct EmbeddedCommunitySourceAdapter {
    service: Arc<CommunityService>,
}

impl EmbeddedCommunitySourceAdapter {
    pub fn new(service: Arc<CommunityService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl FeedSourceAdapter for EmbeddedCommunitySourceAdapter {
    fn source_type(&self) -> &'static str {
        SOURCE_TYPE
    }

    fn clone_box(&self) -> Box<dyn FeedSourceAdapter> {
        Box::new(Self {
            service: Arc::clone(&self.service),
        })
    }

    async fn list_changes(
        &self,
        tenant_id: &str,
        stream_key: &str,
        after_published_at: Option<&str>,
        limit: i64,
    ) -> FeedsServiceResult<Vec<FeedSourceItem>> {
        let (category_id, kind, tag) = query_filters(stream_key);
        let mut items: Vec<FeedSourceItem> = Vec::new();
        let mut page: i64 = 1;
        let page_size = limit.clamp(1, MAX_PAGE_SIZE);
        loop {
            let query = CommunityFeedQuery {
                category_id: category_id.clone(),
                kind: kind.clone(),
                q: None,
                review_state: None,
                tag: tag.clone(),
                page,
                page_size,
                approved_only: true,
            };
            let result = self
                .service
                .list_feed(tenant_id, query)
                .await
                .map_err(|error| {
                    FeedsServiceError::Storage(format!(
                        "embedded community source read failed: {error}"
                    ))
                })?;
            let batch: Vec<FeedSourceItem> = result
                .items
                .into_iter()
                .map(project)
                .filter(
                    |item| match (after_published_at, Some(item.published_at.as_str())) {
                        (Some(after), Some(published)) => published > after,
                        _ => true,
                    },
                )
                .collect();
            let batch_len = batch.len() as i64;
            items.extend(batch);
            if batch_len < page_size || items.len() as i64 >= limit {
                break;
            }
            page += 1;
        }
        items.truncate(limit.max(0) as usize);
        // Deterministic ascending order so incremental sync resumes safely.
        items.sort_by(|a, b| {
            a.published_at
                .cmp(&b.published_at)
                .then(a.source_id.cmp(&b.source_id))
        });
        Ok(items)
    }
}
