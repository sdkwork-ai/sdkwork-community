use std::sync::Arc;

use chrono::Utc;
use sdkwork_community_storage_sqlx::{
    CommunityFeedQuery, CommunityGroupPatch, CommunityMemberPatch, CommunitySqlxStore,
    CommunityStoredCategory, CommunityStoredComment, CommunityStoredEntry, CommunityStoredGroup,
    CommunityStoredGroupQr, CommunityStoredMember, CommunityStoredTier, CommunityTierPatch,
    NewCommunityCategory, NewCommunityComment, NewCommunityEntry, NewCommunityGroup,
    NewCommunityMember, NewCommunityTier, SetCommunityReaction,
};
use sdkwork_id_core::{IdGenerator, SnowflakeIdGenerator};
use sdkwork_utils_rust::{slugify, validated_offset_list_params};

use crate::error::CommunityServiceError;
use crate::integration::{
    CommerceIntegration, CommerceIntegrationConfig, MembershipPackageRegistration,
};

#[derive(Debug, Clone)]
pub struct CommunityCategoryView {
    pub id: String,
    pub tenant_id: String,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub avatar: Option<String>,
    pub owner_id: Option<String>,
    pub member_count: i64,
    pub member_limit: Option<i64>,
    pub post_count: i64,
    pub is_paid: bool,
    pub price: Option<f64>,
    pub revenue_target: Option<f64>,
    pub revenue_raised: f64,
    pub tags: Vec<String>,
    pub tabs: Vec<String>,
    pub priority: i64,
    pub enabled: bool,
    pub is_agent_circle: bool,
    pub is_recommended: bool,
    pub is_joined: bool,
}

#[derive(Debug, Clone)]
pub struct CommunityMemberView {
    pub id: String,
    pub tenant_id: String,
    pub category_id: String,
    pub user_id: String,
    pub user_name: String,
    pub role: String,
    pub status: String,
    pub bio: Option<String>,
    pub tier_id: Option<String>,
    pub tier_name: Option<String>,
    pub membership_expires_at: Option<String>,
    pub agent_level: Option<String>,
    pub last_order_id: Option<String>,
    pub joined_at: String,
}

#[derive(Debug, Clone)]
pub struct CommunityTierView {
    pub id: String,
    pub tenant_id: String,
    pub category_id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub duration_days: i64,
    pub lifetime_price: Option<f64>,
    pub lifetime_package_id: Option<String>,
    pub benefits: Vec<String>,
    pub agent_level: Option<String>,
    pub catalog_package_id: Option<String>,
    pub sort_order: i64,
    pub enabled: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommunityGroupQrView {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CommunityGroupView {
    pub id: String,
    pub tenant_id: String,
    pub category_id: String,
    pub name: String,
    pub platform: String,
    pub description: Option<String>,
    pub member_count: i64,
    pub qr_codes: Vec<CommunityGroupQrView>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct CommunityEntryView {
    pub id: String,
    pub tenant_id: String,
    pub category_id: String,
    pub category_label: Option<String>,
    pub author_id: String,
    pub author_name: String,
    pub slug: String,
    pub kind: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub body: Option<String>,
    pub review_state: String,
    pub is_featured: bool,
    pub is_pinned: bool,
    pub has_accepted_answer: bool,
    pub comment_count: i64,
    pub reaction_count: i64,
    pub share_count: i64,
    pub view_count: i64,
    pub tags: Vec<String>,
    pub media: Vec<String>,
    pub published_at: Option<String>,
    pub last_activity_at: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct CommunityEntryPageView {
    pub items: Vec<CommunityEntryView>,
    pub page: i64,
    pub page_size: i64,
    pub total_items: i64,
}

#[derive(Debug, Clone)]
pub struct CommunityCommentView {
    pub id: String,
    pub tenant_id: String,
    pub entry_id: String,
    pub author_id: String,
    pub author_name: String,
    pub body: String,
    pub review_state: String,
    pub is_accepted_answer: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityEntryCommand {
    pub category_id: String,
    pub kind: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub media: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityCategoryCommand {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<i64>,
    pub enabled: Option<bool>,
}

/// App-facing circle (圈子) creation/update command. `slug` is derived from
/// the title by the service.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityCircleCommand {
    pub title: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub avatar: Option<String>,
    pub is_paid: Option<bool>,
    pub member_limit: Option<i64>,
    pub price: Option<f64>,
    pub revenue_target: Option<f64>,
    pub tags: Option<Vec<String>>,
    pub tabs: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityMemberPatchCommand {
    pub role: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityGroupCommand {
    pub name: String,
    pub platform: String,
    pub description: Option<String>,
    pub member_count: Option<i64>,
    pub qr_codes: Option<Vec<CommunityGroupQrView>>,
}

/// Command to create or update a circle membership tier (会员等级).
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityTierCommand {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub duration_days: Option<i64>,
    pub lifetime_price: Option<f64>,
    pub benefits: Option<Vec<String>>,
    pub agent_level: Option<String>,
    pub sort_order: Option<i64>,
}

/// Official circle operator user id: circles owned by this operator (seeded
/// demo circles) get their tiers auto-published at service startup.
pub const OFFICIAL_CIRCLE_OPERATOR_USER_ID: &str = "sdkwork-official";

/// Lifetime membership is materialized as a 100-year term (the order and
/// membership backends require a finite positive duration).
pub const LIFETIME_MEMBERSHIP_DURATION_DAYS: i64 = 36500;

/// Command to activate a paid circle membership after order payment.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityActivateMembershipCommand {
    pub order_id: String,
    /// Package the purchase used: yearly (catalogPackageId) or lifetime
    /// (lifetimePackageId). Absent → yearly.
    pub package_id: Option<String>,
    pub tier_id: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommunityCommentCommand {
    pub body: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityReactionCommand {
    pub reaction_type: String,
    pub active: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityReactionSetAccepted {
    pub accepted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub reaction_count: i64,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityModerationCommand {
    pub review_state: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityPublicationReadinessView {
    pub ready: bool,
    pub degraded: bool,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityCommandAccepted {
    pub accepted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

pub struct CommunityService {
    store: Arc<CommunitySqlxStore>,
    commerce: Arc<CommerceIntegration>,
    id_generator: Arc<dyn IdGenerator>,
}

/// Default generator for tests and dev bootstrap: snowflake node 0 with the
/// canonical SDKWork epoch. Production hosts inject a database-allocated
/// generator through [`CommunityService::with_id_generator`].
fn default_id_generator() -> Arc<dyn IdGenerator> {
    Arc::new(SnowflakeIdGenerator::new(0).expect("snowflake node 0 must initialize"))
}

impl CommunityService {
    pub fn new(store: Arc<CommunitySqlxStore>) -> Self {
        Self::with_commerce(
            store,
            Arc::new(CommerceIntegration::new(
                CommerceIntegrationConfig::from_env(),
            )),
        )
    }

    pub fn with_commerce(
        store: Arc<CommunitySqlxStore>,
        commerce: Arc<CommerceIntegration>,
    ) -> Self {
        Self::with_id_generator(store, commerce, default_id_generator())
    }

    /// Builds a service with a host-provided snowflake generator and the
    /// environment-configured commerce integration (runtime host path).
    pub fn with_runtime_id_generator(
        store: Arc<CommunitySqlxStore>,
        id_generator: Arc<dyn IdGenerator>,
    ) -> Self {
        Self::with_id_generator(
            store,
            Arc::new(CommerceIntegration::new(
                CommerceIntegrationConfig::from_env(),
            )),
            id_generator,
        )
    }

    pub fn with_id_generator(
        store: Arc<CommunitySqlxStore>,
        commerce: Arc<CommerceIntegration>,
        id_generator: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            store,
            commerce,
            id_generator,
        }
    }

    pub fn commerce(&self) -> &CommerceIntegration {
        &self.commerce
    }

    pub fn id_generator(&self) -> &Arc<dyn IdGenerator> {
        &self.id_generator
    }

    /// Generates the next entity id (snowflake, backend-owned). All community
    /// entity ids — circles, entries, comments, reactions, members, groups,
    /// tiers — come from the injected generator; clients never mint ids.
    fn next_entity_id(&self) -> Result<String, CommunityServiceError> {
        self.id_generator.next_id().map_err(|error| {
            CommunityServiceError::Storage(format!("id generation failed: {error}"))
        })
    }

    pub async fn list_categories(
        &self,
        tenant_id: &str,
    ) -> Result<Vec<CommunityCategoryView>, CommunityServiceError> {
        self.store
            .list_categories(tenant_id)
            .await
            .map(|items| items.into_iter().map(map_category).collect())
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))
    }

    /// Lists enabled circles with the requesting user's membership state
    /// (`is_joined`) so clients render list items without per-circle
    /// `members.current` requests.
    pub async fn list_categories_with_membership(
        &self,
        tenant_id: &str,
        user_id: &str,
    ) -> Result<Vec<CommunityCategoryView>, CommunityServiceError> {
        self.store
            .list_categories_with_membership(tenant_id, user_id)
            .await
            .map(|items| items.into_iter().map(map_category).collect())
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))
    }

    pub async fn list_feed(
        &self,
        tenant_id: &str,
        mut query: CommunityFeedQuery,
    ) -> Result<CommunityEntryPageView, CommunityServiceError> {
        let pagination = validated_offset_list_params(Some(query.page), Some(query.page_size))
            .map_err(|_| {
                CommunityServiceError::InvalidParameter(
                    "page must be at least 1 and page_size must be between 1 and 200".to_owned(),
                )
            })?;
        query.page = pagination.page;
        query.page_size = pagination.page_size;
        self.store
            .list_feed(tenant_id, &query)
            .await
            .map(|result| CommunityEntryPageView {
                items: result.items.into_iter().map(map_entry).collect(),
                page: result.page,
                page_size: result.page_size,
                total_items: result.total_items,
            })
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))
    }

    pub async fn retrieve_entry(
        &self,
        tenant_id: &str,
        entry_id: &str,
        approved_only: bool,
    ) -> Result<CommunityEntryView, CommunityServiceError> {
        let entry = self
            .store
            .retrieve_entry_by_id(tenant_id, entry_id, approved_only)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?
            .ok_or_else(|| {
                CommunityServiceError::NotFound(format!("entry {entry_id} not found"))
            })?;
        Ok(map_entry(entry))
    }

    pub async fn retrieve_entry_by_slug(
        &self,
        tenant_id: &str,
        slug: &str,
    ) -> Result<CommunityEntryView, CommunityServiceError> {
        let entry = self
            .store
            .retrieve_entry_by_slug(tenant_id, slug)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?
            .ok_or_else(|| {
                CommunityServiceError::NotFound(format!("entry slug {slug} not found"))
            })?;
        Ok(map_entry(entry))
    }

    pub async fn create_entry(
        &self,
        tenant_id: &str,
        author_id: &str,
        author_name: &str,
        command: CommunityEntryCommand,
    ) -> Result<CommunityEntryView, CommunityServiceError> {
        validate_entry_command(&command)?;
        let now = Utc::now().to_rfc3339();
        let entry_id = self.next_entity_id()?;
        let slug = slugify(&command.title);
        let category_id = command.category_id.clone();
        let input = NewCommunityEntry {
            id: entry_id.clone(),
            tenant_id: tenant_id.to_owned(),
            category_id: command.category_id,
            author_id: author_id.to_owned(),
            author_name: author_name.to_owned(),
            slug,
            kind: command.kind,
            title: command.title,
            excerpt: command.excerpt.unwrap_or_default(),
            body_markdown: command.body.unwrap_or_default(),
            tags: command.tags,
            media: command.media.unwrap_or_default(),
            now,
        };
        self.store
            .create_entry(input)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.adjust_category_counts(tenant_id, &category_id, 0, 1)
            .await?;
        self.retrieve_entry(tenant_id, &entry_id, false).await
    }

    pub async fn update_entry(
        &self,
        tenant_id: &str,
        entry_id: &str,
        command: CommunityEntryCommand,
    ) -> Result<CommunityEntryView, CommunityServiceError> {
        if command.title.trim().is_empty() && command.category_id.trim().is_empty() {
            return Err(CommunityServiceError::Validation(
                "at least one field is required".to_owned(),
            ));
        }
        self.store
            .update_entry(
                tenant_id,
                entry_id,
                &sdkwork_community_storage_sqlx::CommunityEntryPatch {
                    category_id: (!command.category_id.trim().is_empty())
                        .then_some(command.category_id),
                    kind: (!command.kind.trim().is_empty()).then_some(command.kind),
                    title: (!command.title.trim().is_empty()).then_some(command.title),
                    excerpt: command.excerpt,
                    body: command.body,
                    tags: Some(command.tags),
                    media: command.media,
                },
            )
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.retrieve_entry(tenant_id, entry_id, false).await
    }

    pub async fn publication_readiness(
        &self,
        tenant_id: &str,
        entry_id: &str,
    ) -> Result<CommunityPublicationReadinessView, CommunityServiceError> {
        let entry = self.retrieve_entry(tenant_id, entry_id, false).await?;
        let mut issues = Vec::new();
        if entry.review_state == "flagged" {
            issues.push("flagged".to_owned());
        } else if entry.review_state == "pending-review" {
            issues.push("pending-review".to_owned());
        } else if entry.review_state == "rejected" {
            issues.push("rejected".to_owned());
        }
        if entry.title.trim().is_empty() {
            issues.push("missing-title".to_owned());
        }
        if entry.category_id.trim().is_empty() {
            issues.push("missing-category".to_owned());
        }
        if entry.body.as_deref().unwrap_or("").trim().is_empty() {
            issues.push("missing-body".to_owned());
        }
        let excerpt_required = matches!(entry.kind.as_str(), "resource" | "service");
        if excerpt_required && entry.excerpt.as_deref().unwrap_or("").trim().is_empty() {
            issues.push("missing-excerpt".to_owned());
        }
        if entry
            .tags
            .iter()
            .filter(|tag| !tag.trim().is_empty())
            .count()
            < 1
        {
            issues.push("missing-tags".to_owned());
        }
        let ready = issues.iter().all(|issue| {
            !matches!(
                issue.as_str(),
                "pending-review"
                    | "flagged"
                    | "missing-body"
                    | "missing-category"
                    | "missing-excerpt"
                    | "missing-tags"
                    | "missing-title"
                    | "rejected"
            )
        });
        Ok(CommunityPublicationReadinessView {
            ready,
            degraded: ready && !issues.is_empty(),
            issues,
        })
    }

    pub async fn list_recommendations(
        &self,
        tenant_id: &str,
        entry_id: &str,
    ) -> Result<Vec<CommunityEntryView>, CommunityServiceError> {
        let source = self.retrieve_entry(tenant_id, entry_id, true).await?;
        let mut candidates = self
            .list_feed(
                tenant_id,
                CommunityFeedQuery {
                    approved_only: true,
                    page: 1,
                    page_size: 100,
                    ..CommunityFeedQuery::default()
                },
            )
            .await?;
        candidates
            .items
            .retain(|candidate| candidate.id != source.id);
        candidates.items.sort_by(|left, right| {
            recommendation_score(&source, right)
                .cmp(&recommendation_score(&source, left))
                .then_with(|| right.updated_at.cmp(&left.updated_at))
        });
        candidates.items.truncate(10);
        Ok(candidates.items)
    }

    pub async fn list_comments(
        &self,
        tenant_id: &str,
        entry_id: &str,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<CommunityCommentView>, i64), CommunityServiceError> {
        let offset = (page - 1).max(0) * page_size;
        let items = self
            .store
            .list_comments(tenant_id, entry_id, page_size, offset)
            .await
            .map(|items| items.into_iter().map(map_comment).collect())
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        let total = self
            .store
            .count_comments(tenant_id, entry_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        Ok((items, total))
    }

    pub async fn create_comment(
        &self,
        tenant_id: &str,
        entry_id: &str,
        author_id: &str,
        author_name: &str,
        command: CommunityCommentCommand,
    ) -> Result<CommunityCommentView, CommunityServiceError> {
        if command.body.trim().is_empty() {
            return Err(CommunityServiceError::Validation(
                "comment body is required".to_owned(),
            ));
        }
        let _ = self.retrieve_entry(tenant_id, entry_id, false).await?;
        let now = Utc::now().to_rfc3339();
        let comment_id = self.next_entity_id()?;
        self.store
            .create_comment(NewCommunityComment {
                id: comment_id.clone(),
                tenant_id: tenant_id.to_owned(),
                entry_id: entry_id.to_owned(),
                author_id: author_id.to_owned(),
                author_name: author_name.to_owned(),
                body_markdown: command.body,
                now,
            })
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.store
            .retrieve_comment(tenant_id, comment_id.as_str())
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?
            .map(map_comment)
            .ok_or_else(|| CommunityServiceError::Storage("created comment not found".to_owned()))
    }

    pub async fn set_reaction(
        &self,
        tenant_id: &str,
        entry_id: &str,
        user_id: &str,
        command: CommunityReactionCommand,
    ) -> Result<CommunityReactionSetAccepted, CommunityServiceError> {
        let reaction_type = command.reaction_type.trim();
        if reaction_type.is_empty() {
            return Err(CommunityServiceError::Validation(
                "reactionType is required".to_owned(),
            ));
        }
        let _ = self.retrieve_entry(tenant_id, entry_id, false).await?;
        let now = Utc::now().to_rfc3339();
        let reaction_count = self
            .store
            .set_reaction(SetCommunityReaction {
                id: self.next_entity_id()?,
                tenant_id: tenant_id.to_owned(),
                entry_id: entry_id.to_owned(),
                user_id: user_id.to_owned(),
                reaction_type: reaction_type.to_owned(),
                active: command.active,
                now,
            })
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        Ok(CommunityReactionSetAccepted {
            accepted: true,
            resource_id: Some(entry_id.to_owned()),
            status: Some(if command.active { "active" } else { "inactive" }.to_owned()),
            reaction_count,
        })
    }

    pub async fn delete_entry_for_author(
        &self,
        tenant_id: &str,
        author_id: &str,
        entry_id: &str,
    ) -> Result<CommunityCommandAccepted, CommunityServiceError> {
        let entry = self.retrieve_entry(tenant_id, entry_id, false).await?;
        if entry.author_id != author_id {
            return Err(CommunityServiceError::Unauthorized(
                "only the author can delete this entry".to_owned(),
            ));
        }
        self.delete_entry(tenant_id, entry_id).await
    }

    pub async fn create_category(
        &self,
        tenant_id: &str,
        command: CommunityCategoryCommand,
    ) -> Result<CommunityCategoryView, CommunityServiceError> {
        if command.slug.trim().is_empty() || command.title.trim().is_empty() {
            return Err(CommunityServiceError::Validation(
                "slug and title are required".to_owned(),
            ));
        }
        let now = Utc::now().to_rfc3339();
        let category_id = self.next_entity_id()?;
        self.store
            .create_category(NewCommunityCategory {
                id: category_id.clone(),
                tenant_id: tenant_id.to_owned(),
                slug: slugify(&command.slug),
                title: command.title,
                description: command.description,
                cover_image: None,
                avatar: None,
                owner_id: None,
                member_limit: None,
                is_paid: false,
                price: None,
                revenue_target: None,
                tags: Vec::new(),
                tabs: Vec::new(),
                priority: command.priority.unwrap_or(0),
                enabled: command.enabled.unwrap_or(true),
                now,
            })
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.store
            .list_categories(tenant_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?
            .into_iter()
            .find(|category| category.id == category_id)
            .map(map_category)
            .ok_or_else(|| CommunityServiceError::Storage("created category not found".to_owned()))
    }

    pub async fn update_category(
        &self,
        tenant_id: &str,
        category_id: &str,
        command: CommunityCategoryCommand,
    ) -> Result<CommunityCategoryView, CommunityServiceError> {
        self.store
            .update_category(
                tenant_id,
                category_id,
                &sdkwork_community_storage_sqlx::CommunityCategoryPatch {
                    slug: (!command.slug.trim().is_empty()).then_some(slugify(&command.slug)),
                    title: (!command.title.trim().is_empty()).then_some(command.title),
                    description: command.description,
                    cover_image: None,
                    avatar: None,
                    owner_id: None,
                    member_count: None,
                    member_limit: None,
                    post_count: None,
                    is_paid: None,
                    revenue_raised: None,
                    revenue_target: None,
                    price: None,
                    tags: None,
                    tabs: None,
                    priority: command.priority,
                    enabled: command.enabled,
                },
            )
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.store
            .list_categories(tenant_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?
            .into_iter()
            .find(|category| category.id == category_id)
            .map(map_category)
            .ok_or_else(|| {
                CommunityServiceError::NotFound(format!("category {category_id} not found"))
            })
    }

    /// Deletes a circle (app API). Only the circle owner may delete it;
    /// membership rows cascade with the category.
    pub async fn delete_circle(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
    ) -> Result<CommunityCommandAccepted, CommunityServiceError> {
        let member = self
            .current_member(tenant_id, category_id, actor_user_id)
            .await?;
        match member.as_ref().map(|member| member.role.as_str()) {
            Some("owner") => {}
            _ => {
                return Err(CommunityServiceError::Unauthorized(
                    "owner role required to delete the circle".to_owned(),
                ));
            }
        }
        self.delete_category(tenant_id, category_id).await
    }

    pub async fn delete_category(
        &self,
        tenant_id: &str,
        category_id: &str,
    ) -> Result<CommunityCommandAccepted, CommunityServiceError> {
        let deleted = self
            .store
            .delete_category(tenant_id, category_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        if !deleted {
            return Err(CommunityServiceError::NotFound(format!(
                "category {category_id} not found"
            )));
        }
        Ok(CommunityCommandAccepted {
            accepted: true,
            resource_id: Some(category_id.to_owned()),
            status: Some("deleted".to_owned()),
        })
    }

    /// Creates a circle (圈子): a category owned by the creator with an owner
    /// membership row.
    pub async fn create_circle(
        &self,
        tenant_id: &str,
        user_id: &str,
        display_name: &str,
        command: CommunityCircleCommand,
    ) -> Result<CommunityCategoryView, CommunityServiceError> {
        if command.title.trim().is_empty() {
            return Err(CommunityServiceError::Validation(
                "circle title is required".to_owned(),
            ));
        }
        let now = Utc::now().to_rfc3339();
        let category_id = self.next_entity_id()?;
        self.store
            .create_category(NewCommunityCategory {
                id: category_id.clone(),
                tenant_id: tenant_id.to_owned(),
                slug: slugify(&command.title),
                title: command.title,
                description: command.description,
                cover_image: command.cover_image,
                avatar: command.avatar,
                owner_id: Some(user_id.to_owned()),
                member_limit: validate_member_limit(command.member_limit)?,
                is_paid: command.is_paid.unwrap_or(false),
                price: command.price,
                revenue_target: validate_revenue_target(command.revenue_target)?,
                tags: command.tags.unwrap_or_default(),
                tabs: command.tabs.unwrap_or_default(),
                priority: 0,
                enabled: true,
                now: now.clone(),
            })
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.store
            .create_member(NewCommunityMember {
                id: self.next_entity_id()?,
                tenant_id: tenant_id.to_owned(),
                category_id: category_id.clone(),
                user_id: user_id.to_owned(),
                user_name: display_name.to_owned(),
                role: "owner".to_owned(),
                bio: None,
                now,
            })
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.adjust_category_counts(tenant_id, &category_id, 1, 0)
            .await?;
        // The creator is an owner member; return the circle with is_joined.
        self.retrieve_category_with_membership(tenant_id, &category_id, user_id)
            .await
    }

    pub async fn update_circle(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        command: CommunityCircleCommand,
    ) -> Result<CommunityCategoryView, CommunityServiceError> {
        self.require_manager(tenant_id, category_id, actor_user_id)
            .await?;
        self.store
            .update_category(
                tenant_id,
                category_id,
                &sdkwork_community_storage_sqlx::CommunityCategoryPatch {
                    slug: None,
                    title: (!command.title.trim().is_empty()).then_some(command.title),
                    description: command.description,
                    cover_image: command.cover_image,
                    avatar: command.avatar,
                    owner_id: None,
                    member_count: None,
                    member_limit: validate_member_limit(command.member_limit)?,
                    post_count: None,
                    is_paid: command.is_paid,
                    revenue_raised: None,
                    revenue_target: validate_revenue_target(command.revenue_target)?,
                    price: command.price,
                    tags: command.tags,
                    tabs: command.tabs,
                    priority: None,
                    enabled: None,
                },
            )
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.retrieve_category(tenant_id, category_id).await
    }

    pub async fn retrieve_category(
        &self,
        tenant_id: &str,
        category_id: &str,
    ) -> Result<CommunityCategoryView, CommunityServiceError> {
        self.store
            .list_categories(tenant_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?
            .into_iter()
            .find(|category| category.id == category_id)
            .map(map_category)
            .ok_or_else(|| {
                CommunityServiceError::NotFound(format!("category {category_id} not found"))
            })
    }

    /// Retrieves a single circle together with the requesting user's
    /// membership state (`is_joined`), matching the list behavior.
    pub async fn retrieve_category_with_membership(
        &self,
        tenant_id: &str,
        category_id: &str,
        user_id: &str,
    ) -> Result<CommunityCategoryView, CommunityServiceError> {
        self.store
            .retrieve_category_with_membership(tenant_id, category_id, user_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?
            .map(map_category)
            .ok_or_else(|| {
                CommunityServiceError::NotFound(format!("category {category_id} not found"))
            })
    }

    pub async fn list_members(
        &self,
        tenant_id: &str,
        category_id: &str,
    ) -> Result<Vec<CommunityMemberView>, CommunityServiceError> {
        self.store
            .list_members(tenant_id, category_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))
            .map(|items| items.into_iter().map(map_member).collect())
    }

    pub async fn current_member(
        &self,
        tenant_id: &str,
        category_id: &str,
        user_id: &str,
    ) -> Result<Option<CommunityMemberView>, CommunityServiceError> {
        self.store
            .current_member(tenant_id, category_id, user_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))
            .map(|member| member.map(map_member))
    }

    pub async fn join_category(
        &self,
        tenant_id: &str,
        category_id: &str,
        user_id: &str,
        display_name: &str,
    ) -> Result<CommunityMemberView, CommunityServiceError> {
        self.retrieve_category(tenant_id, category_id).await?;
        if let Some(member) = self.current_member(tenant_id, category_id, user_id).await? {
            return Ok(member);
        }
        // Circles with purchasable membership tiers require a paid membership;
        // direct joins are rejected so the purchase flow stays authoritative.
        if !self
            .store
            .list_tiers(tenant_id, category_id, true)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?
            .is_empty()
        {
            return Err(CommunityServiceError::Validation(
                "this circle requires a paid membership; please purchase a membership tier"
                    .to_owned(),
            ));
        }
        self.ensure_member_capacity(tenant_id, category_id).await?;
        let now = Utc::now().to_rfc3339();
        let member_id = self.next_entity_id()?;
        self.store
            .create_member(NewCommunityMember {
                id: member_id.clone(),
                tenant_id: tenant_id.to_owned(),
                category_id: category_id.to_owned(),
                user_id: user_id.to_owned(),
                user_name: display_name.to_owned(),
                role: "member".to_owned(),
                bio: None,
                now,
            })
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.adjust_category_counts(tenant_id, category_id, 1, 0)
            .await?;
        self.current_member(tenant_id, category_id, user_id)
            .await?
            .ok_or_else(|| CommunityServiceError::Storage("joined membership not found".to_owned()))
    }

    pub async fn update_member(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        member_id: &str,
        command: CommunityMemberPatchCommand,
    ) -> Result<CommunityMemberView, CommunityServiceError> {
        self.require_manager(tenant_id, category_id, actor_user_id)
            .await?;
        let member = self
            .list_members(tenant_id, category_id)
            .await?
            .into_iter()
            .find(|member| member.id == member_id)
            .ok_or_else(|| {
                CommunityServiceError::NotFound(format!("member {member_id} not found"))
            })?;
        if member.role == "owner" && command.role.as_deref() == Some("member") {
            return Err(CommunityServiceError::Validation(
                "the owner role cannot be demoted".to_owned(),
            ));
        }
        self.store
            .update_member(
                tenant_id,
                category_id,
                member_id,
                &CommunityMemberPatch {
                    role: command.role,
                    status: command.status,
                    tier_id: None,
                    tier_name: None,
                    membership_expires_at: None,
                    agent_level: None,
                    last_order_id: None,
                },
            )
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.list_members(tenant_id, category_id)
            .await?
            .into_iter()
            .find(|member| member.id == member_id)
            .ok_or_else(|| CommunityServiceError::NotFound(format!("member {member_id} not found")))
    }

    pub async fn remove_member(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        member_id: &str,
    ) -> Result<CommunityCommandAccepted, CommunityServiceError> {
        self.require_manager(tenant_id, category_id, actor_user_id)
            .await?;
        let deleted = self
            .store
            .delete_member(tenant_id, category_id, member_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        if !deleted {
            return Err(CommunityServiceError::NotFound(format!(
                "member {member_id} not found"
            )));
        }
        self.adjust_category_counts(tenant_id, category_id, -1, 0)
            .await?;
        Ok(CommunityCommandAccepted {
            accepted: true,
            resource_id: Some(member_id.to_owned()),
            status: Some("removed".to_owned()),
        })
    }

    pub async fn list_groups(
        &self,
        tenant_id: &str,
        category_id: &str,
    ) -> Result<Vec<CommunityGroupView>, CommunityServiceError> {
        self.store
            .list_groups(tenant_id, category_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))
            .map(|items| items.into_iter().map(map_group).collect())
    }

    pub async fn create_group(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        command: CommunityGroupCommand,
    ) -> Result<CommunityGroupView, CommunityServiceError> {
        self.require_member(tenant_id, category_id, actor_user_id)
            .await?;
        if command.name.trim().is_empty() {
            return Err(CommunityServiceError::Validation(
                "group name is required".to_owned(),
            ));
        }
        let now = Utc::now().to_rfc3339();
        let group_id = self.next_entity_id()?;
        self.store
            .create_group(NewCommunityGroup {
                id: group_id.clone(),
                tenant_id: tenant_id.to_owned(),
                category_id: category_id.to_owned(),
                name: command.name,
                platform: command.platform,
                description: command.description,
                member_count: command.member_count.unwrap_or(0),
                qr_codes: command
                    .qr_codes
                    .unwrap_or_default()
                    .into_iter()
                    .map(|qr| CommunityStoredGroupQr {
                        url: qr.url,
                        description: qr.description,
                    })
                    .collect(),
                now,
            })
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.list_groups(tenant_id, category_id)
            .await?
            .into_iter()
            .find(|group| group.id == group_id)
            .ok_or_else(|| CommunityServiceError::Storage("created group not found".to_owned()))
    }

    pub async fn update_group(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        group_id: &str,
        command: CommunityGroupCommand,
    ) -> Result<CommunityGroupView, CommunityServiceError> {
        self.require_manager(tenant_id, category_id, actor_user_id)
            .await?;
        self.store
            .update_group(
                tenant_id,
                category_id,
                group_id,
                &CommunityGroupPatch {
                    name: (!command.name.trim().is_empty()).then_some(command.name),
                    platform: (!command.platform.trim().is_empty()).then_some(command.platform),
                    description: command.description,
                    member_count: command.member_count,
                    qr_codes: command.qr_codes.map(|qrs| {
                        qrs.into_iter()
                            .map(|qr| CommunityStoredGroupQr {
                                url: qr.url,
                                description: qr.description,
                            })
                            .collect()
                    }),
                },
            )
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.list_groups(tenant_id, category_id)
            .await?
            .into_iter()
            .find(|group| group.id == group_id)
            .ok_or_else(|| CommunityServiceError::NotFound(format!("group {group_id} not found")))
    }

    pub async fn delete_group(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        group_id: &str,
    ) -> Result<CommunityCommandAccepted, CommunityServiceError> {
        self.require_manager(tenant_id, category_id, actor_user_id)
            .await?;
        let deleted = self
            .store
            .delete_group(tenant_id, category_id, group_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        if !deleted {
            return Err(CommunityServiceError::NotFound(format!(
                "group {group_id} not found"
            )));
        }
        Ok(CommunityCommandAccepted {
            accepted: true,
            resource_id: Some(group_id.to_owned()),
            status: Some("deleted".to_owned()),
        })
    }

    async fn adjust_category_counts(
        &self,
        tenant_id: &str,
        category_id: &str,
        member_delta: i64,
        post_delta: i64,
    ) -> Result<(), CommunityServiceError> {
        let category = self.retrieve_category(tenant_id, category_id).await?;
        self.store
            .update_category(
                tenant_id,
                category_id,
                &sdkwork_community_storage_sqlx::CommunityCategoryPatch {
                    slug: None,
                    title: None,
                    description: None,
                    cover_image: None,
                    avatar: None,
                    owner_id: None,
                    member_count: Some((category.member_count + member_delta).max(0)),
                    member_limit: None,
                    post_count: Some((category.post_count + post_delta).max(0)),
                    revenue_raised: None,
                    revenue_target: None,
                    is_paid: None,
                    price: None,
                    tags: None,
                    tabs: None,
                    priority: None,
                    enabled: None,
                },
            )
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))
    }

    async fn require_member(
        &self,
        tenant_id: &str,
        category_id: &str,
        user_id: &str,
    ) -> Result<(), CommunityServiceError> {
        let member = self.current_member(tenant_id, category_id, user_id).await?;
        if member.is_none() {
            return Err(CommunityServiceError::Unauthorized(
                "membership required".to_owned(),
            ));
        }
        Ok(())
    }

    /// Rejects joins/activations when the circle has a member limit and it is
    /// already reached (NULL limit means unlimited).
    async fn ensure_member_capacity(
        &self,
        tenant_id: &str,
        category_id: &str,
    ) -> Result<(), CommunityServiceError> {
        let category = self.retrieve_category(tenant_id, category_id).await?;
        if let Some(limit) = category.member_limit {
            if category.member_count >= limit {
                return Err(CommunityServiceError::Conflict(
                    "circle member limit reached".to_owned(),
                ));
            }
        }
        Ok(())
    }

    /// Rejects activation when the raise target would be exceeded by the
    /// tier price (funding/angel-round cap; NULL target means no cap).
    async fn ensure_revenue_capacity(
        &self,
        tenant_id: &str,
        category_id: &str,
        tier_price: f64,
    ) -> Result<(), CommunityServiceError> {
        let category = self.retrieve_category(tenant_id, category_id).await?;
        if let Some(target) = category.revenue_target {
            if category.revenue_raised + tier_price > target {
                return Err(CommunityServiceError::Conflict(
                    "circle revenue target reached; purchases are closed".to_owned(),
                ));
            }
        }
        Ok(())
    }

    /// Adds the paid amount to the circle's raised revenue after activation
    /// (atomic SQL increment so concurrent activations never lose amounts).
    async fn accumulate_revenue(
        &self,
        tenant_id: &str,
        category_id: &str,
        paid_amount: f64,
    ) -> Result<(), CommunityServiceError> {
        self.store
            .accumulate_category_revenue(tenant_id, category_id, paid_amount)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))
    }

    /// Keeps the circle's display price in sync with the lowest purchasable
    /// tier price after tier changes (purchase surface always settles on the
    /// tier price; the category price is the 'from' price shown in lists).
    async fn sync_category_price(
        &self,
        tenant_id: &str,
        category_id: &str,
    ) -> Result<(), CommunityServiceError> {
        let tiers = self.list_tiers(tenant_id, category_id, true).await?;
        if tiers.is_empty() {
            return Ok(());
        }
        let lowest = tiers
            .iter()
            .map(|tier| tier.price)
            .fold(f64::INFINITY, f64::min);
        let category = self.retrieve_category(tenant_id, category_id).await?;
        if category.price != Some(lowest) {
            self.store
                .update_category(
                    tenant_id,
                    category_id,
                    &sdkwork_community_storage_sqlx::CommunityCategoryPatch {
                        slug: None,
                        title: None,
                        description: None,
                        cover_image: None,
                        avatar: None,
                        owner_id: None,
                        member_count: None,
                        member_limit: None,
                        post_count: None,
                        is_paid: None,
                        price: Some(lowest),
                        revenue_raised: None,
                        revenue_target: None,
                        tags: None,
                        tabs: None,
                        priority: None,
                        enabled: None,
                    },
                )
                .await
                .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        }
        Ok(())
    }

    async fn require_manager(
        &self,
        tenant_id: &str,
        category_id: &str,
        user_id: &str,
    ) -> Result<(), CommunityServiceError> {
        let member = self.current_member(tenant_id, category_id, user_id).await?;
        match member.as_ref().map(|member| member.role.as_str()) {
            Some("owner" | "admin") => Ok(()),
            _ => Err(CommunityServiceError::Unauthorized(
                "owner or admin role required".to_owned(),
            )),
        }
    }

    /// Lists circle membership tiers. `enabled_only` hides unpublished tiers
    /// from the purchase surface while owners manage the full list.
    pub async fn list_tiers(
        &self,
        tenant_id: &str,
        category_id: &str,
        enabled_only: bool,
    ) -> Result<Vec<CommunityTierView>, CommunityServiceError> {
        self.store
            .list_tiers(tenant_id, category_id, enabled_only)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))
            .map(|items| items.into_iter().map(map_tier).collect())
    }

    /// Creates an unpublished membership tier (owner/admin). Publishing is a
    /// separate step so the tier only becomes purchasable once its catalog
    /// package has been registered.
    pub async fn create_tier(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        command: CommunityTierCommand,
    ) -> Result<CommunityTierView, CommunityServiceError> {
        self.require_manager(tenant_id, category_id, actor_user_id)
            .await?;
        if command.name.trim().is_empty() {
            return Err(CommunityServiceError::Validation(
                "tier name is required".to_owned(),
            ));
        }
        if command.price < 0.0 {
            return Err(CommunityServiceError::Validation(
                "tier price must not be negative".to_owned(),
            ));
        }
        let now = Utc::now().to_rfc3339();
        let tier_id = self.next_entity_id()?;
        self.store
            .create_tier(NewCommunityTier {
                id: tier_id.clone(),
                tenant_id: tenant_id.to_owned(),
                category_id: category_id.to_owned(),
                name: command.name,
                description: command.description,
                price: command.price,
                duration_days: command.duration_days.unwrap_or(365),
                lifetime_price: command.lifetime_price,
                benefits: command.benefits.unwrap_or_default(),
                agent_level: command.agent_level,
                sort_order: command.sort_order.unwrap_or(0),
                now,
            })
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        let tier = self.retrieve_tier(tenant_id, category_id, &tier_id).await?;
        self.sync_category_price(tenant_id, category_id).await?;
        Ok(tier)
    }

    pub async fn update_tier(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        tier_id: &str,
        command: CommunityTierCommand,
    ) -> Result<CommunityTierView, CommunityServiceError> {
        self.require_manager(tenant_id, category_id, actor_user_id)
            .await?;
        let existing = self.retrieve_tier(tenant_id, category_id, tier_id).await?;
        if command.price < 0.0 {
            return Err(CommunityServiceError::Validation(
                "tier price must not be negative".to_owned(),
            ));
        }
        self.store
            .update_tier(
                tenant_id,
                category_id,
                tier_id,
                &CommunityTierPatch {
                    name: (!command.name.trim().is_empty()).then_some(command.name),
                    description: command.description,
                    price: Some(command.price),
                    duration_days: command.duration_days,
                    lifetime_price: command.lifetime_price,
                    lifetime_package_id: None,
                    benefits: command.benefits,
                    agent_level: command.agent_level,
                    catalog_package_id: None,
                    sort_order: command.sort_order,
                    enabled: None,
                },
            )
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        let _ = existing;
        let tier = self.retrieve_tier(tenant_id, category_id, tier_id).await?;
        self.sync_category_price(tenant_id, category_id).await?;
        Ok(tier)
    }

    /// Idempotently publishes every unpublished tier of the official
    /// operator's paid circles (`owner_id = 'sdkwork-official'`), so the
    /// seeded multi-price purchase surface works out of the box. Runs once at
    /// service startup; already-published tiers are skipped and user-created
    /// circles keep owner publishing control.
    pub async fn publish_official_circle_tiers(&self) -> Result<usize, CommunityServiceError> {
        let categories = self
            .store
            .list_official_paid_categories()
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        let mut published = 0usize;
        for category in categories {
            let tiers = self
                .list_tiers(&category.tenant_id, &category.id, false)
                .await?;
            for tier in tiers {
                if tier.enabled
                    && tier.catalog_package_id.is_some()
                    && (tier.lifetime_price.is_none() || tier.lifetime_package_id.is_some())
                {
                    continue;
                }
                self.publish_tier(
                    &category.tenant_id,
                    OFFICIAL_CIRCLE_OPERATOR_USER_ID,
                    &category.id,
                    &tier.id,
                )
                .await?;
                published += 1;
            }
        }
        Ok(published)
    }

    /// Publishes a tier: registers the membership package on the membership
    /// backend (auto-assigned external id becomes the order `packageId`) and
    /// makes the tier purchasable.
    pub async fn publish_tier(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        tier_id: &str,
    ) -> Result<CommunityTierView, CommunityServiceError> {
        self.require_manager(tenant_id, category_id, actor_user_id)
            .await?;
        let tier = self.retrieve_tier(tenant_id, category_id, tier_id).await?;
        if tier.enabled
            && tier.catalog_package_id.is_some()
            && (tier.lifetime_price.is_none() || tier.lifetime_package_id.is_some())
        {
            return Ok(tier);
        }
        let category = self.retrieve_category(tenant_id, category_id).await?;
        let mut patch = CommunityTierPatch {
            name: None,
            description: None,
            price: None,
            duration_days: None,
            lifetime_price: None,
            lifetime_package_id: None,
            benefits: None,
            agent_level: None,
            catalog_package_id: None,
            sort_order: None,
            enabled: Some(true),
        };

        // Yearly package (skipped when already registered).
        if tier.catalog_package_id.is_none() {
            let registered = self
                .commerce
                .register_membership_package(MembershipPackageRegistration {
                    category: "community".to_owned(),
                    code: format!("community-tier-{}", tier.id.replace('-', "")),
                    package_group_id: "package-group-circle-membership".to_owned(),
                    plan_id: "plan-circle-membership".to_owned(),
                    name: format!("{} · {}", category.title, tier.name),
                    price_amount: format!("{:.2}", tier.price),
                    currency_code: "CNY".to_owned(),
                    duration_days: tier.duration_days,
                    // Full price: the membership backend requires a discount
                    // percent (1-100) and 100 means no discount.
                    discount: 100,
                    status: "active".to_owned(),
                })
                .await
                .map_err(CommunityServiceError::Integration)?;
            patch.catalog_package_id = Some(registered.external_id.to_string());
        }

        // Lifetime package (100-year approximation) when the tier offers one.
        if let Some(lifetime_price) = tier.lifetime_price {
            if tier.lifetime_package_id.is_none() {
                let lifetime = self
                    .commerce
                    .register_membership_package(MembershipPackageRegistration {
                        category: "community".to_owned(),
                        code: format!("community-tier-{}-lifetime", tier.id.replace('-', "")),
                        package_group_id: "package-group-circle-membership".to_owned(),
                        plan_id: "plan-circle-membership".to_owned(),
                        name: format!("{} · {}（终身）", category.title, tier.name),
                        price_amount: format!("{lifetime_price:.2}"),
                        currency_code: "CNY".to_owned(),
                        duration_days: LIFETIME_MEMBERSHIP_DURATION_DAYS,
                        discount: 100,
                        status: "active".to_owned(),
                    })
                    .await
                    .map_err(CommunityServiceError::Integration)?;
                patch.lifetime_package_id = Some(lifetime.external_id.to_string());
            }
        }

        self.store
            .update_tier(tenant_id, category_id, tier_id, &patch)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        let tier = self.retrieve_tier(tenant_id, category_id, tier_id).await?;
        self.sync_category_price(tenant_id, category_id).await?;
        Ok(tier)
    }

    pub async fn unpublish_tier(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        tier_id: &str,
    ) -> Result<CommunityTierView, CommunityServiceError> {
        self.require_manager(tenant_id, category_id, actor_user_id)
            .await?;
        self.store
            .update_tier(
                tenant_id,
                category_id,
                tier_id,
                &CommunityTierPatch {
                    name: None,
                    description: None,
                    price: None,
                    duration_days: None,
                    lifetime_price: None,
                    lifetime_package_id: None,
                    benefits: None,
                    agent_level: None,
                    catalog_package_id: None,
                    sort_order: None,
                    enabled: Some(false),
                },
            )
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        let tier = self.retrieve_tier(tenant_id, category_id, tier_id).await?;
        self.sync_category_price(tenant_id, category_id).await?;
        Ok(tier)
    }

    pub async fn delete_tier(
        &self,
        tenant_id: &str,
        actor_user_id: &str,
        category_id: &str,
        tier_id: &str,
    ) -> Result<CommunityCommandAccepted, CommunityServiceError> {
        self.require_manager(tenant_id, category_id, actor_user_id)
            .await?;
        let deleted = self
            .store
            .delete_tier(tenant_id, category_id, tier_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        if !deleted {
            return Err(CommunityServiceError::NotFound(format!(
                "tier {tier_id} not found"
            )));
        }
        self.sync_category_price(tenant_id, category_id).await?;
        Ok(CommunityCommandAccepted {
            accepted: true,
            resource_id: Some(tier_id.to_owned()),
            status: Some("deleted".to_owned()),
        })
    }

    /// Activates a paid circle membership after order payment: verifies the
    /// order with the order backend, upserts the member and applies the tier
    /// with its expiry.
    pub async fn activate_membership(
        &self,
        tenant_id: &str,
        category_id: &str,
        user_id: &str,
        display_name: &str,
        command: CommunityActivateMembershipCommand,
    ) -> Result<CommunityMemberView, CommunityServiceError> {
        let tier = self
            .retrieve_tier(tenant_id, category_id, &command.tier_id)
            .await?;
        if !tier.enabled || tier.catalog_package_id.is_none() {
            return Err(CommunityServiceError::Validation(
                "membership tier is not purchasable".to_owned(),
            ));
        }
        let verification = self
            .commerce
            .verify_order_paid(&command.order_id)
            .await
            .map_err(CommunityServiceError::Integration)?;
        if !verification.paid {
            return Err(CommunityServiceError::Validation(
                "order is not paid".to_owned(),
            ));
        }
        // Accumulate the actual paid amount from the order (the tier price may
        // have changed since the package was registered).
        let paid_amount = verification.paid_amount.unwrap_or(tier.price);
        let now = Utc::now().to_rfc3339();
        // A purchase through the lifetime package grants the membership
        // forever (membership_expires_at stays NULL); any other package uses
        // the tier duration.
        let is_lifetime = tier.lifetime_package_id.is_some()
            && command.package_id.as_deref() == tier.lifetime_package_id.as_deref();
        let expires_at: Option<String> = if is_lifetime {
            None
        } else {
            Some(
                chrono::DateTime::parse_from_rfc3339(&now)
                    .map(|value| value + chrono::Duration::days(tier.duration_days))
                    .unwrap_or_else(|_| {
                        chrono::DateTime::parse_from_rfc3339("2030-01-01T00:00:00Z")
                            .expect("static fallback expiry")
                    })
                    .to_rfc3339(),
            )
        };

        // Idempotency: replaying the same paid order must not double-count
        // revenue nor extend the membership twice, and must succeed even
        // after the raise target has been reached.
        if let Some(member) = self.current_member(tenant_id, category_id, user_id).await? {
            if member.last_order_id.as_deref() == Some(command.order_id.as_str()) {
                return Ok(member);
            }
        }
        self.ensure_revenue_capacity(tenant_id, category_id, paid_amount)
            .await?;
        if self
            .current_member(tenant_id, category_id, user_id)
            .await?
            .is_none()
        {
            self.ensure_member_capacity(tenant_id, category_id).await?;
            self.store
                .create_member(NewCommunityMember {
                    id: self.next_entity_id()?,
                    tenant_id: tenant_id.to_owned(),
                    category_id: category_id.to_owned(),
                    user_id: user_id.to_owned(),
                    user_name: display_name.to_owned(),
                    role: "member".to_owned(),
                    bio: None,
                    now: now.clone(),
                })
                .await
                .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
            self.adjust_category_counts(tenant_id, category_id, 1, 0)
                .await?;
        }
        let member = self
            .current_member(tenant_id, category_id, user_id)
            .await?
            .ok_or_else(|| {
                CommunityServiceError::Storage("activated membership not found".to_owned())
            })?;
        self.store
            .update_member(
                tenant_id,
                category_id,
                &member.id,
                &CommunityMemberPatch {
                    role: None,
                    status: None,
                    tier_id: Some(tier.id.clone()),
                    tier_name: Some(tier.name),
                    membership_expires_at: expires_at,
                    agent_level: tier.agent_level.clone(),
                    last_order_id: Some(command.order_id.clone()),
                },
            )
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.accumulate_revenue(tenant_id, category_id, paid_amount)
            .await?;
        self.current_member(tenant_id, category_id, user_id)
            .await?
            .ok_or_else(|| {
                CommunityServiceError::Storage("activated membership not found".to_owned())
            })
    }

    async fn retrieve_tier(
        &self,
        tenant_id: &str,
        category_id: &str,
        tier_id: &str,
    ) -> Result<CommunityTierView, CommunityServiceError> {
        self.store
            .list_tiers(tenant_id, category_id, false)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?
            .into_iter()
            .find(|tier| tier.id == tier_id)
            .map(map_tier)
            .ok_or_else(|| CommunityServiceError::NotFound(format!("tier {tier_id} not found")))
    }

    pub async fn update_moderation(
        &self,
        tenant_id: &str,
        entry_id: &str,
        actor_user_id: &str,
        command: CommunityModerationCommand,
    ) -> Result<CommunityEntryView, CommunityServiceError> {
        self.store
            .update_moderation(
                tenant_id,
                entry_id,
                actor_user_id,
                &sdkwork_community_storage_sqlx::CommunityModerationPatch {
                    review_state: command.review_state,
                    reason: command.reason,
                },
            )
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.retrieve_entry(tenant_id, entry_id, false).await
    }

    pub async fn set_featured(
        &self,
        tenant_id: &str,
        entry_id: &str,
        featured: bool,
    ) -> Result<CommunityEntryView, CommunityServiceError> {
        self.store
            .set_featured(tenant_id, entry_id, featured)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.retrieve_entry(tenant_id, entry_id, false).await
    }

    pub async fn set_pinned(
        &self,
        tenant_id: &str,
        entry_id: &str,
        pinned: bool,
    ) -> Result<CommunityEntryView, CommunityServiceError> {
        self.store
            .set_pinned(tenant_id, entry_id, pinned)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        self.retrieve_entry(tenant_id, entry_id, false).await
    }

    pub async fn delete_entry(
        &self,
        tenant_id: &str,
        entry_id: &str,
    ) -> Result<CommunityCommandAccepted, CommunityServiceError> {
        let deleted = self
            .store
            .delete_entry(tenant_id, entry_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        if !deleted {
            return Err(CommunityServiceError::NotFound(format!(
                "entry {entry_id} not found"
            )));
        }
        Ok(CommunityCommandAccepted {
            accepted: true,
            resource_id: Some(entry_id.to_owned()),
            status: Some("deleted".to_owned()),
        })
    }

    pub async fn list_moderation_queue(
        &self,
        tenant_id: &str,
    ) -> Result<Vec<CommunityEntryView>, CommunityServiceError> {
        self.store
            .list_moderation_queue(tenant_id)
            .await
            .map(|items| items.into_iter().map(map_entry).collect())
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))
    }

    pub async fn rebuild_recommendations(
        &self,
        tenant_id: &str,
    ) -> Result<CommunityCommandAccepted, CommunityServiceError> {
        let rebuilt = self
            .store
            .rebuild_recommendations(tenant_id)
            .await
            .map_err(|error| CommunityServiceError::Storage(error.to_string()))?;
        Ok(CommunityCommandAccepted {
            accepted: true,
            resource_id: None,
            status: Some(format!("rebuilt:{rebuilt}")),
        })
    }
}

fn map_category(category: CommunityStoredCategory) -> CommunityCategoryView {
    CommunityCategoryView {
        id: category.id,
        tenant_id: category.tenant_id,
        slug: category.slug,
        title: category.title,
        description: category.description,
        cover_image: category.cover_image,
        avatar: category.avatar,
        owner_id: category.owner_id,
        member_count: category.member_count,
        member_limit: category.member_limit,
        post_count: category.post_count,
        is_paid: category.is_paid,
        price: category.price,
        revenue_target: category.revenue_target,
        revenue_raised: category.revenue_raised,
        tags: category.tags,
        tabs: category.tabs,
        priority: category.priority,
        enabled: category.enabled,
        is_agent_circle: category.is_agent_circle,
        is_recommended: category.is_recommended,
        is_joined: category.is_joined,
    }
}

fn map_member(member: CommunityStoredMember) -> CommunityMemberView {
    CommunityMemberView {
        id: member.id,
        tenant_id: member.tenant_id,
        category_id: member.category_id,
        user_id: member.user_id,
        user_name: member.user_name,
        role: member.role,
        status: member.status,
        bio: member.bio,
        tier_id: member.tier_id,
        tier_name: member.tier_name,
        membership_expires_at: member.membership_expires_at,
        agent_level: member.agent_level,
        last_order_id: member.last_order_id,
        joined_at: member.joined_at,
    }
}

fn map_tier(tier: CommunityStoredTier) -> CommunityTierView {
    CommunityTierView {
        id: tier.id,
        tenant_id: tier.tenant_id,
        category_id: tier.category_id,
        name: tier.name,
        description: tier.description,
        price: tier.price,
        duration_days: tier.duration_days,
        lifetime_price: tier.lifetime_price,
        lifetime_package_id: tier.lifetime_package_id,
        benefits: tier.benefits,
        agent_level: tier.agent_level,
        catalog_package_id: tier.catalog_package_id,
        sort_order: tier.sort_order,
        enabled: tier.enabled,
    }
}

fn map_group(group: CommunityStoredGroup) -> CommunityGroupView {
    CommunityGroupView {
        id: group.id,
        tenant_id: group.tenant_id,
        category_id: group.category_id,
        name: group.name,
        platform: group.platform,
        description: group.description,
        member_count: group.member_count,
        qr_codes: group
            .qr_codes
            .into_iter()
            .map(|qr| CommunityGroupQrView {
                url: qr.url,
                description: qr.description,
            })
            .collect(),
        created_at: group.created_at,
        updated_at: group.updated_at,
    }
}

fn map_entry(entry: CommunityStoredEntry) -> CommunityEntryView {
    CommunityEntryView {
        id: entry.id,
        tenant_id: entry.tenant_id,
        category_id: entry.category_id,
        category_label: None,
        author_id: entry.author_id,
        author_name: entry.author_name,
        slug: entry.slug,
        kind: entry.kind,
        title: entry.title,
        excerpt: Some(entry.excerpt),
        body: Some(entry.body_markdown),
        review_state: entry.review_state,
        is_featured: entry.is_featured,
        is_pinned: entry.is_pinned,
        has_accepted_answer: entry.has_accepted_answer,
        comment_count: entry.comment_count,
        reaction_count: entry.reaction_count,
        share_count: entry.share_count,
        view_count: entry.view_count,
        tags: entry.tags,
        media: entry.media,
        published_at: entry.published_at,
        last_activity_at: entry.last_activity_at,
        updated_at: entry.updated_at,
    }
}

fn map_comment(comment: CommunityStoredComment) -> CommunityCommentView {
    CommunityCommentView {
        id: comment.id,
        tenant_id: comment.tenant_id,
        entry_id: comment.entry_id,
        author_id: comment.author_id,
        author_name: comment.author_name,
        body: comment.body_markdown,
        review_state: comment.review_state,
        is_accepted_answer: comment.is_accepted_answer,
        created_at: comment.created_at,
        updated_at: comment.updated_at,
    }
}

fn validate_member_limit(value: Option<i64>) -> Result<Option<i64>, CommunityServiceError> {
    match value {
        None | Some(0) => Ok(None),
        Some(limit) if limit < 0 => Err(CommunityServiceError::Validation(
            "member limit must be positive or empty".to_owned(),
        )),
        Some(limit) => Ok(Some(limit)),
    }
}

fn validate_revenue_target(value: Option<f64>) -> Result<Option<f64>, CommunityServiceError> {
    match value {
        None | Some(0.0) => Ok(None),
        Some(target) if target < 0.0 => Err(CommunityServiceError::Validation(
            "revenue target must be positive or empty".to_owned(),
        )),
        Some(target) => Ok(Some(target)),
    }
}

fn validate_entry_command(command: &CommunityEntryCommand) -> Result<(), CommunityServiceError> {
    if command.category_id.trim().is_empty() {
        return Err(CommunityServiceError::Validation(
            "categoryId is required".to_owned(),
        ));
    }
    if command.title.trim().is_empty() {
        return Err(CommunityServiceError::Validation(
            "title is required".to_owned(),
        ));
    }
    if command.kind.trim().is_empty() {
        return Err(CommunityServiceError::Validation(
            "kind is required".to_owned(),
        ));
    }
    Ok(())
}

fn recommendation_score(source: &CommunityEntryView, candidate: &CommunityEntryView) -> i64 {
    let mut score = 0;
    if source.category_id == candidate.category_id {
        score += 3;
    }
    if source.kind == candidate.kind {
        score += 2;
    }
    if source.author_id == candidate.author_id {
        score += 1;
    }
    if candidate.is_featured {
        score += 2;
    }
    if candidate.has_accepted_answer {
        score += 1;
    }
    score
        + candidate
            .tags
            .iter()
            .filter(|tag| source.tags.contains(tag))
            .count() as i64
}
