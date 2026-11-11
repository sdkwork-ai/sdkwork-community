//! PostgreSQL query implementations for Community authoritative persistence.

use std::collections::HashMap;

use sqlx::{PgPool, Row};

use super::{
    CommunityCategoryPatch, CommunityEntryPatch, CommunityFeedQuery, CommunityGroupPatch,
    CommunityMemberPatch, CommunityModerationPatch, CommunityStoredCategory,
    CommunityStoredComment, CommunityStoredEntry, CommunityStoredEntryPage, CommunityStoredGroup,
    CommunityStoredGroupQr, CommunityStoredMember, CommunityStoredTier, CommunityTierPatch,
    NewCommunityCategory, NewCommunityComment, NewCommunityEntry, NewCommunityGroup,
    NewCommunityMember, NewCommunityTier,
};

fn category_from_row(row: &sqlx::postgres::PgRow) -> CommunityStoredCategory {
    CommunityStoredCategory {
        id: string_cell(row, "id"),
        tenant_id: string_cell(row, "tenant_id"),
        slug: string_cell(row, "slug"),
        title: string_cell(row, "title"),
        description: optional_string_cell(row, "description"),
        cover_image: optional_string_cell(row, "cover_image"),
        avatar: optional_string_cell(row, "avatar"),
        owner_id: optional_string_cell(row, "owner_id"),
        member_count: integer_cell(row, "member_count"),
        member_limit: optional_integer_cell(row, "member_limit"),
        post_count: integer_cell(row, "post_count"),
        is_paid: bool_cell(row, "is_paid"),
        price: optional_f64_cell(row, "price"),
        revenue_target: optional_f64_cell(row, "revenue_target"),
        revenue_raised: optional_f64_cell(row, "revenue_raised").unwrap_or(0.0),
        tags: text_array_cell(row, "tags"),
        tabs: text_array_cell(row, "tabs"),
        priority: integer_cell(row, "priority"),
        enabled: bool_cell(row, "enabled"),
        is_agent_circle: bool_cell(row, "is_agent_circle"),
        is_recommended: bool_cell(row, "is_recommended"),
        is_joined: bool_cell(row, "is_joined"),
    }
}

/// Lists paid circles owned by the official operator
/// (`owner_id = 'sdkwork-official'`), across tenants. Used by the startup
/// bootstrap that publishes the seeded official tiers so the multi-price
/// purchase surface works out of the box.
pub async fn list_official_paid_categories(
    pool: &PgPool,
) -> Result<Vec<CommunityStoredCategory>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, tenant_id, slug, title, description, cover_image, avatar, owner_id,
               member_count, member_limit, post_count, is_paid, price::float8,
               revenue_target::float8, revenue_raised::float8, tags, tabs, priority, enabled,
               is_agent_circle, is_recommended, FALSE AS is_joined
        FROM community_category
        WHERE owner_id = 'sdkwork-official' AND is_paid = TRUE AND enabled = TRUE
        ORDER BY is_recommended DESC, is_paid DESC, priority DESC, slug ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.iter().map(category_from_row).collect())
}

pub async fn list_categories(
    pool: &PgPool,
    tenant_id: &str,
) -> Result<Vec<CommunityStoredCategory>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, tenant_id, slug, title, description, cover_image, avatar, owner_id,
               member_count, member_limit, post_count, is_paid, price::float8,
               revenue_target::float8, revenue_raised::float8, tags, tabs, priority, enabled,
               is_agent_circle, FALSE AS is_joined
        FROM community_category
        WHERE tenant_id = $1 AND enabled = TRUE
        ORDER BY is_recommended DESC, is_paid DESC, priority DESC, slug ASC
        "#,
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.iter().map(category_from_row).collect())
}

/// Lists enabled circles together with the requesting user's membership
/// state, so list rendering never needs one `members.current` request per
/// circle (N+1).
pub async fn list_categories_with_membership(
    pool: &PgPool,
    tenant_id: &str,
    user_id: &str,
) -> Result<Vec<CommunityStoredCategory>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT c.id, c.tenant_id, c.slug, c.title, c.description, c.cover_image,
               c.avatar, c.owner_id, c.member_count, c.member_limit, c.post_count,
               c.is_paid, c.price::float8, c.revenue_target::float8,
               c.revenue_raised::float8, c.tags, c.tabs, c.priority, c.enabled,
               c.is_agent_circle, c.is_recommended, (cm.id IS NOT NULL) AS is_joined
        FROM community_category c
        LEFT JOIN community_member cm
          ON cm.category_id = c.id
         AND cm.tenant_id = c.tenant_id
         AND cm.user_id = $2
         AND cm.status = 'active'
        WHERE c.tenant_id = $1 AND c.enabled = TRUE
        ORDER BY c.is_recommended DESC, c.is_paid DESC, c.priority DESC, c.slug ASC
        "#,
    )
    .bind(tenant_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.iter().map(category_from_row).collect())
}

/// Retrieves a single enabled circle together with the requesting user's
/// membership state (used by the app GET /categories/{categoryId} route).
pub async fn retrieve_category_with_membership(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    user_id: &str,
) -> Result<Option<CommunityStoredCategory>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT c.id, c.tenant_id, c.slug, c.title, c.description, c.cover_image,
               c.avatar, c.owner_id, c.member_count, c.member_limit, c.post_count,
               c.is_paid, c.price::float8, c.revenue_target::float8,
               c.revenue_raised::float8, c.tags, c.tabs, c.priority, c.enabled,
               c.is_agent_circle, (cm.id IS NOT NULL) AS is_joined
        FROM community_category c
        LEFT JOIN community_member cm
          ON cm.category_id = c.id
         AND cm.tenant_id = c.tenant_id
         AND cm.user_id = $3
         AND cm.status = 'active'
        WHERE c.tenant_id = $1 AND c.id = $2 AND c.enabled = TRUE
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .bind(category_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|row| category_from_row(&row)))
}

pub async fn count_comments(
    pool: &PgPool,
    tenant_id: &str,
    entry_id: &str,
) -> Result<i64, sqlx::Error> {
    let total = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM community_comment
        WHERE tenant_id = $1 AND entry_id = $2
        "#,
    )
    .bind(tenant_id)
    .bind(entry_id)
    .fetch_one(pool)
    .await?;
    Ok(total)
}

pub async fn retrieve_comment(
    pool: &PgPool,
    tenant_id: &str,
    comment_id: &str,
) -> Result<Option<CommunityStoredComment>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, tenant_id, entry_id, author_id, author_name, body_markdown, review_state,
               is_accepted_answer, created_at, updated_at
        FROM community_comment
        WHERE tenant_id = $1 AND id = $2
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .bind(comment_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|row| CommunityStoredComment {
        id: string_cell(&row, "id"),
        tenant_id: string_cell(&row, "tenant_id"),
        entry_id: string_cell(&row, "entry_id"),
        author_id: string_cell(&row, "author_id"),
        author_name: string_cell(&row, "author_name"),
        body_markdown: string_cell(&row, "body_markdown"),
        review_state: string_cell(&row, "review_state"),
        is_accepted_answer: bool_cell(&row, "is_accepted_answer"),
        created_at: string_cell(&row, "created_at"),
        updated_at: optional_string_cell(&row, "updated_at"),
    }))
}

pub async fn create_category(
    pool: &PgPool,
    input: NewCommunityCategory,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO community_category
            (id, tenant_id, slug, title, description, cover_image, avatar, owner_id,
             member_count, member_limit, post_count, is_paid, price, revenue_target,
             tags, tabs, priority, enabled, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12::numeric, $13::numeric,
                $14::text[], $15::jsonb, $16, $17, $18, $19)
        "#,
    )
    .bind(input.id)
    .bind(input.tenant_id)
    .bind(input.slug)
    .bind(input.title)
    .bind(input.description)
    .bind(input.cover_image)
    .bind(input.avatar)
    .bind(input.owner_id)
    .bind(0i64)
    .bind(0i64)
    .bind(input.member_limit)
    .bind(input.is_paid)
    .bind(input.price)
    .bind(input.revenue_target)
    .bind(&input.tags)
    .bind(serde_json::to_string(&input.tabs).unwrap_or_else(|_| "[]".to_owned()))
    .bind(input.priority)
    .bind(input.enabled)
    .bind(&input.now)
    .bind(&input.now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_category(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    patch: &CommunityCategoryPatch,
) -> Result<(), sqlx::Error> {
    let existing = list_categories(pool, tenant_id).await?;
    let Some(existing) = existing.into_iter().find(|item| item.id == category_id) else {
        return Ok(());
    };
    sqlx::query(
        r#"
        UPDATE community_category
        SET slug = $1, title = $2, description = $3, cover_image = $4, avatar = $5,
            owner_id = $6, member_count = $7, member_limit = $8, post_count = $9,
            is_paid = $10, price = $11::numeric, revenue_raised = $12::numeric,
            revenue_target = $13::numeric, tags = $14::text[], tabs = $15::jsonb,
            priority = $16, enabled = $17, updated_at = $18
        WHERE tenant_id = $19 AND id = $20
        "#,
    )
    .bind(patch.slug.as_ref().unwrap_or(&existing.slug))
    .bind(patch.title.as_ref().unwrap_or(&existing.title))
    .bind(patch.description.as_ref().or(existing.description.as_ref()))
    .bind(patch.cover_image.as_ref().or(existing.cover_image.as_ref()))
    .bind(patch.avatar.as_ref().or(existing.avatar.as_ref()))
    .bind(patch.owner_id.as_ref().or(existing.owner_id.as_ref()))
    .bind(patch.member_count.unwrap_or(existing.member_count))
    .bind(patch.member_limit.or(existing.member_limit))
    .bind(patch.post_count.unwrap_or(existing.post_count))
    .bind(patch.is_paid.unwrap_or(existing.is_paid))
    .bind(patch.price.or(existing.price))
    .bind(patch.revenue_raised.or(Some(existing.revenue_raised)))
    .bind(patch.revenue_target.or(existing.revenue_target))
    .bind(patch.tags.as_ref().unwrap_or(&existing.tags))
    .bind(
        serde_json::to_string(patch.tabs.as_ref().unwrap_or(&existing.tabs))
            .unwrap_or_else(|_| "[]".to_owned()),
    )
    .bind(patch.priority.unwrap_or(existing.priority))
    .bind(patch.enabled.unwrap_or(existing.enabled))
    .bind(chrono::Utc::now().to_rfc3339())
    .bind(tenant_id)
    .bind(category_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_category(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM community_category WHERE tenant_id = $1 AND id = $2")
        .bind(tenant_id)
        .bind(category_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn create_entry(pool: &PgPool, input: NewCommunityEntry) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"
        INSERT INTO community_entry
            (id, tenant_id, category_id, author_id, author_name, slug, kind, title, excerpt,
             review_state, is_featured, is_pinned, has_accepted_answer, comment_count,
             reaction_count, share_count, view_count, media, published_at, last_activity_at, created_at, updated_at)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'draft', FALSE, FALSE, FALSE, 0, 0, 0, 0,
             $10::jsonb, NULL, $11, $12, $13)
        "#,
    )
    .bind(&input.id)
    .bind(&input.tenant_id)
    .bind(&input.category_id)
    .bind(&input.author_id)
    .bind(&input.author_name)
    .bind(&input.slug)
    .bind(&input.kind)
    .bind(&input.title)
    .bind(&input.excerpt)
    .bind(serde_json::to_string(&input.media).unwrap_or_else(|_| "[]".to_owned()))
    .bind(&input.now)
    .bind(&input.now)
    .bind(&input.now)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO community_entry_body (entry_id, body_markdown, body_format, content_checksum, updated_at)
        VALUES ($1, $2, 'markdown', NULL, $3)
        "#,
    )
    .bind(&input.id)
    .bind(&input.body_markdown)
    .bind(&input.now)
    .execute(&mut *tx)
    .await?;
    upsert_entry_tags(&mut tx, &input).await?;
    tx.commit().await
}

pub async fn update_entry(
    pool: &PgPool,
    tenant_id: &str,
    entry_id: &str,
    patch: &CommunityEntryPatch,
) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    if let Some(entry) = retrieve_entry_by_id(pool, tenant_id, entry_id, false).await? {
        sqlx::query(
            r#"
            UPDATE community_entry
            SET category_id = $1, kind = $2, title = $3, excerpt = $4,
                media = $5::jsonb, updated_at = $6
            WHERE tenant_id = $7 AND id = $8
            "#,
        )
        .bind(patch.category_id.as_ref().unwrap_or(&entry.category_id))
        .bind(patch.kind.as_ref().unwrap_or(&entry.kind))
        .bind(patch.title.as_ref().unwrap_or(&entry.title))
        .bind(patch.excerpt.as_ref().unwrap_or(&entry.excerpt))
        .bind(
            serde_json::to_string(patch.media.as_ref().unwrap_or(&entry.media))
                .unwrap_or_else(|_| "[]".to_owned()),
        )
        .bind(&now)
        .bind(tenant_id)
        .bind(entry_id)
        .execute(pool)
        .await?;
        if let Some(body) = &patch.body {
            sqlx::query(
                "UPDATE community_entry_body SET body_markdown = $1, updated_at = $2 WHERE entry_id = $3",
            )
            .bind(body)
            .bind(&now)
            .bind(entry_id)
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}

pub async fn list_feed(
    pool: &PgPool,
    tenant_id: &str,
    query: &CommunityFeedQuery,
) -> Result<CommunityStoredEntryPage, sqlx::Error> {
    let review_state = if query.approved_only {
        Some("approved".to_owned())
    } else {
        query.review_state.clone()
    };
    let q = query
        .q
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase);
    let tag = query
        .tag
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase);
    let page = query.page.max(1);
    let page_size = query.page_size.clamp(1, 200);
    let offset = (page - 1) * page_size;
    let count_row = sqlx::query(
        r#"
        SELECT COUNT(*) AS total_items
        FROM community_entry e
        JOIN community_entry_body b ON b.entry_id = e.id
        WHERE e.tenant_id = $1
          AND ($2::text IS NULL OR e.review_state = $2)
          AND ($3::text IS NULL OR e.category_id = $3)
          AND ($4::text IS NULL OR e.kind = $4)
          AND (
            $5::text IS NULL
            OR POSITION($5 IN LOWER(e.title)) > 0
            OR POSITION($5 IN LOWER(COALESCE(e.excerpt, ''))) > 0
            OR EXISTS (
              SELECT 1
              FROM community_entry_tag et
              JOIN community_tag t ON t.id = et.tag_id
              WHERE et.entry_id = e.id
                AND t.tenant_id = e.tenant_id
                AND POSITION($5 IN LOWER(t.slug)) > 0
            )
          )
          AND (
            $6::text IS NULL
            OR EXISTS (
              SELECT 1
              FROM community_entry_tag et
              JOIN community_tag t ON t.id = et.tag_id
              WHERE et.entry_id = e.id
                AND t.tenant_id = e.tenant_id
                AND LOWER(t.slug) = $6
            )
          )
        "#,
    )
    .bind(tenant_id)
    .bind(review_state.as_deref())
    .bind(query.category_id.as_deref())
    .bind(query.kind.as_deref())
    .bind(q.as_deref())
    .bind(tag.as_deref())
    .fetch_one(pool)
    .await?;
    let total_items = integer_cell(&count_row, "total_items");
    let rows = sqlx::query(
        r#"
        SELECT e.id, e.tenant_id, e.category_id, e.author_id, e.author_name, e.slug, e.kind,
               e.title, e.excerpt, b.body_markdown, e.review_state, e.is_featured, e.is_pinned,
               e.has_accepted_answer, e.comment_count, e.reaction_count, e.share_count,
               e.view_count, e.media, e.published_at, e.last_activity_at, e.updated_at
        FROM community_entry e
        JOIN community_entry_body b ON b.entry_id = e.id
        WHERE e.tenant_id = $1
          AND ($2::text IS NULL OR e.review_state = $2)
          AND ($3::text IS NULL OR e.category_id = $3)
          AND ($4::text IS NULL OR e.kind = $4)
          AND (
            $5::text IS NULL
            OR POSITION($5 IN LOWER(e.title)) > 0
            OR POSITION($5 IN LOWER(COALESCE(e.excerpt, ''))) > 0
            OR EXISTS (
              SELECT 1
              FROM community_entry_tag et
              JOIN community_tag t ON t.id = et.tag_id
              WHERE et.entry_id = e.id
                AND t.tenant_id = e.tenant_id
                AND POSITION($5 IN LOWER(t.slug)) > 0
            )
          )
          AND (
            $6::text IS NULL
            OR EXISTS (
              SELECT 1
              FROM community_entry_tag et
              JOIN community_tag t ON t.id = et.tag_id
              WHERE et.entry_id = e.id
                AND t.tenant_id = e.tenant_id
                AND LOWER(t.slug) = $6
            )
          )
        ORDER BY e.is_pinned DESC, e.last_activity_at DESC, e.published_at DESC, e.slug ASC
        LIMIT $7 OFFSET $8
        "#,
    )
    .bind(tenant_id)
    .bind(review_state.as_deref())
    .bind(query.category_id.as_deref())
    .bind(query.kind.as_deref())
    .bind(q.as_deref())
    .bind(tag.as_deref())
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        items.push(entry_from_row(pool, row).await?);
    }
    Ok(CommunityStoredEntryPage {
        items,
        page,
        page_size,
        total_items,
    })
}

pub async fn retrieve_entry_by_id(
    pool: &PgPool,
    tenant_id: &str,
    entry_id: &str,
    approved_only: bool,
) -> Result<Option<CommunityStoredEntry>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT e.id, e.tenant_id, e.category_id, e.author_id, e.author_name, e.slug, e.kind,
               e.title, e.excerpt, b.body_markdown, e.review_state, e.is_featured, e.is_pinned,
               e.has_accepted_answer, e.comment_count, e.reaction_count, e.share_count,
               e.view_count, e.media, e.published_at, e.last_activity_at, e.updated_at
        FROM community_entry e
        JOIN community_entry_body b ON b.entry_id = e.id
        WHERE e.tenant_id = $1 AND e.id = $2
          AND ($3 = FALSE OR e.review_state = 'approved')
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .bind(entry_id)
    .bind(approved_only)
    .fetch_optional(pool)
    .await?;
    match row {
        Some(row) => entry_from_row(pool, row).await.map(Some),
        None => Ok(None),
    }
}

pub async fn retrieve_entry_by_slug(
    pool: &PgPool,
    tenant_id: &str,
    slug: &str,
) -> Result<Option<CommunityStoredEntry>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT e.id, e.tenant_id, e.category_id, e.author_id, e.author_name, e.slug, e.kind,
               e.title, e.excerpt, b.body_markdown, e.review_state, e.is_featured, e.is_pinned,
               e.has_accepted_answer, e.comment_count, e.reaction_count, e.share_count,
               e.view_count, e.media, e.published_at, e.last_activity_at, e.updated_at
        FROM community_entry e
        JOIN community_entry_body b ON b.entry_id = e.id
        WHERE e.tenant_id = $1 AND e.slug = $2 AND e.review_state = 'approved'
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .bind(slug)
    .fetch_optional(pool)
    .await?;
    match row {
        Some(row) => entry_from_row(pool, row).await.map(Some),
        None => Ok(None),
    }
}

pub async fn list_comments(
    pool: &PgPool,
    tenant_id: &str,
    entry_id: &str,
    page_size: i64,
    offset: i64,
) -> Result<Vec<CommunityStoredComment>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, tenant_id, entry_id, author_id, author_name, body_markdown, review_state,
               is_accepted_answer, created_at, updated_at
        FROM community_comment
        WHERE tenant_id = $1 AND entry_id = $2
        ORDER BY created_at ASC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(tenant_id)
    .bind(entry_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .iter()
        .map(|row| CommunityStoredComment {
            id: string_cell(row, "id"),
            tenant_id: string_cell(row, "tenant_id"),
            entry_id: string_cell(row, "entry_id"),
            author_id: string_cell(row, "author_id"),
            author_name: string_cell(row, "author_name"),
            body_markdown: string_cell(row, "body_markdown"),
            review_state: string_cell(row, "review_state"),
            is_accepted_answer: bool_cell(row, "is_accepted_answer"),
            created_at: string_cell(row, "created_at"),
            updated_at: optional_string_cell(row, "updated_at"),
        })
        .collect())
}

pub async fn create_comment(pool: &PgPool, input: NewCommunityComment) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO community_comment
            (id, tenant_id, entry_id, author_id, author_name, body_markdown, review_state,
             is_accepted_answer, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, 'approved', FALSE, $7, $8)
        "#,
    )
    .bind(&input.id)
    .bind(&input.tenant_id)
    .bind(&input.entry_id)
    .bind(&input.author_id)
    .bind(&input.author_name)
    .bind(&input.body_markdown)
    .bind(&input.now)
    .bind(&input.now)
    .execute(pool)
    .await?;
    sqlx::query(
        "UPDATE community_entry SET comment_count = comment_count + 1, updated_at = $1 WHERE id = $2",
    )
    .bind(&input.now)
    .bind(&input.entry_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_moderation(
    pool: &PgPool,
    tenant_id: &str,
    entry_id: &str,
    actor_user_id: &str,
    patch: &CommunityModerationPatch,
) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    let before = retrieve_entry_by_id(pool, tenant_id, entry_id, false)
        .await?
        .map(|entry| entry.review_state)
        .unwrap_or_else(|| "draft".to_owned());
    sqlx::query(
        r#"
        UPDATE community_entry
        SET review_state = $1,
            published_at = CASE WHEN $2 = 'approved' THEN COALESCE(published_at, $3) ELSE published_at END,
            last_activity_at = $4, updated_at = $5
        WHERE tenant_id = $6 AND id = $7
        "#,
    )
    .bind(&patch.review_state)
    .bind(&patch.review_state)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .bind(tenant_id)
    .bind(entry_id)
    .execute(pool)
    .await?;
    sqlx::query(
        r#"
        INSERT INTO community_moderation_event
            (id, tenant_id, entry_id, action, actor_user_id, reason, before_state, after_state, created_at)
        VALUES ($1, $2, $3, 'moderate', $4, $5, $6, $7, $8)
        "#,
    )
    .bind(format!("moderation_{tenant_id}_{entry_id}_{now}"))
    .bind(tenant_id)
    .bind(entry_id)
    .bind(actor_user_id)
    .bind(&patch.reason)
    .bind(before)
    .bind(&patch.review_state)
    .bind(&now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_featured(
    pool: &PgPool,
    tenant_id: &str,
    entry_id: &str,
    featured: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE community_entry SET is_featured = $1, updated_at = $2 WHERE tenant_id = $3 AND id = $4")
        .bind(featured)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(tenant_id)
        .bind(entry_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_pinned(
    pool: &PgPool,
    tenant_id: &str,
    entry_id: &str,
    pinned: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE community_entry SET is_pinned = $1, updated_at = $2 WHERE tenant_id = $3 AND id = $4")
        .bind(pinned)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(tenant_id)
        .bind(entry_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_reaction(
    pool: &PgPool,
    input: &super::SetCommunityReaction,
) -> Result<i64, sqlx::Error> {
    if input.active {
        let inserted = sqlx::query(
            r#"
            INSERT INTO community_reaction
                (id, tenant_id, entry_id, user_id, reaction_type, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (tenant_id, entry_id, user_id, reaction_type) DO NOTHING
            "#,
        )
        .bind(&input.id)
        .bind(&input.tenant_id)
        .bind(&input.entry_id)
        .bind(&input.user_id)
        .bind(&input.reaction_type)
        .bind(&input.now)
        .execute(pool)
        .await?;
        if inserted.rows_affected() > 0 {
            sqlx::query(
                r#"
                UPDATE community_entry
                SET reaction_count = reaction_count + 1,
                    last_activity_at = $1,
                    updated_at = $2
                WHERE tenant_id = $3 AND id = $4
                "#,
            )
            .bind(&input.now)
            .bind(&input.now)
            .bind(&input.tenant_id)
            .bind(&input.entry_id)
            .execute(pool)
            .await?;
        }
    } else {
        let deleted = sqlx::query(
            r#"
            DELETE FROM community_reaction
            WHERE tenant_id = $1 AND entry_id = $2 AND user_id = $3 AND reaction_type = $4
            "#,
        )
        .bind(&input.tenant_id)
        .bind(&input.entry_id)
        .bind(&input.user_id)
        .bind(&input.reaction_type)
        .execute(pool)
        .await?;
        if deleted.rows_affected() > 0 {
            sqlx::query(
                r#"
                UPDATE community_entry
                SET reaction_count = GREATEST(reaction_count - 1, 0),
                    last_activity_at = $1,
                    updated_at = $2
                WHERE tenant_id = $3 AND id = $4
                "#,
            )
            .bind(&input.now)
            .bind(&input.now)
            .bind(&input.tenant_id)
            .bind(&input.entry_id)
            .execute(pool)
            .await?;
        }
    }

    let row =
        sqlx::query("SELECT reaction_count FROM community_entry WHERE tenant_id = $1 AND id = $2")
            .bind(&input.tenant_id)
            .bind(&input.entry_id)
            .fetch_one(pool)
            .await?;
    Ok(integer_cell(&row, "reaction_count"))
}

pub async fn delete_entry(
    pool: &PgPool,
    tenant_id: &str,
    entry_id: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM community_entry WHERE tenant_id = $1 AND id = $2")
        .bind(tenant_id)
        .bind(entry_id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn list_moderation_queue(
    pool: &PgPool,
    tenant_id: &str,
) -> Result<Vec<CommunityStoredEntry>, sqlx::Error> {
    list_feed(
        pool,
        tenant_id,
        &CommunityFeedQuery {
            review_state: Some("pending-review".to_owned()),
            page: 1,
            page_size: 200,
            approved_only: false,
            ..CommunityFeedQuery::default()
        },
    )
    .await
    .map(|page| page.items)
}

pub async fn rebuild_recommendations(pool: &PgPool, tenant_id: &str) -> Result<i64, sqlx::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    let mut count = 0_i64;
    let page_size = 200_i64;
    let mut page = 1_i64;
    loop {
        let result = list_feed(
            pool,
            tenant_id,
            &CommunityFeedQuery {
                page,
                page_size,
                approved_only: true,
                ..CommunityFeedQuery::default()
            },
        )
        .await?;
        let total_items = result.total_items;
        if result.items.is_empty() {
            break;
        }
        for entry in result.items {
            sqlx::query(
                r#"
                INSERT INTO community_recommendation_snapshot
                    (id, tenant_id, source_entry_id, target_entry_id, score, created_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                ON CONFLICT (id) DO UPDATE SET score = excluded.score, created_at = excluded.created_at
                "#,
            )
            .bind(format!("rec_{}_{}", entry.id, tenant_id))
            .bind(tenant_id)
            .bind(&entry.id)
            .bind(&entry.id)
            .bind(1_i64)
            .bind(&now)
            .execute(pool)
            .await?;
            count += 1;
        }
        if page * page_size >= total_items {
            break;
        }
        page += 1;
    }
    Ok(count)
}

async fn entry_from_row(
    pool: &PgPool,
    row: sqlx::postgres::PgRow,
) -> Result<CommunityStoredEntry, sqlx::Error> {
    let entry_id = string_cell(&row, "id");
    let tags = entry_tags(pool, &entry_id).await?;
    Ok(CommunityStoredEntry {
        id: entry_id,
        tenant_id: string_cell(&row, "tenant_id"),
        category_id: string_cell(&row, "category_id"),
        author_id: string_cell(&row, "author_id"),
        author_name: string_cell(&row, "author_name"),
        slug: string_cell(&row, "slug"),
        kind: string_cell(&row, "kind"),
        title: string_cell(&row, "title"),
        excerpt: string_cell(&row, "excerpt"),
        body_markdown: string_cell(&row, "body_markdown"),
        review_state: string_cell(&row, "review_state"),
        is_featured: bool_cell(&row, "is_featured"),
        is_pinned: bool_cell(&row, "is_pinned"),
        has_accepted_answer: bool_cell(&row, "has_accepted_answer"),
        comment_count: integer_cell(&row, "comment_count"),
        reaction_count: integer_cell(&row, "reaction_count"),
        share_count: integer_cell(&row, "share_count"),
        view_count: integer_cell(&row, "view_count"),
        media: text_array_cell(&row, "media"),
        tags,
        published_at: optional_string_cell(&row, "published_at"),
        last_activity_at: optional_string_cell(&row, "last_activity_at"),
        updated_at: string_cell(&row, "updated_at"),
    })
}

async fn entry_tags(pool: &PgPool, entry_id: &str) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT t.slug
        FROM community_entry_tag et
        JOIN community_tag t ON t.id = et.tag_id
        WHERE et.entry_id = $1
        ORDER BY t.slug ASC
        "#,
    )
    .bind(entry_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.iter().map(|row| string_cell(row, "slug")).collect())
}

async fn upsert_entry_tags(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    input: &NewCommunityEntry,
) -> Result<(), sqlx::Error> {
    let mut tags = input
        .tags
        .iter()
        .map(|tag| sdkwork_utils_rust::slugify(tag))
        .filter(|tag| !tag.is_empty())
        .collect::<Vec<_>>();
    tags.sort();
    tags.dedup();
    for tag in tags {
        let tag_id = format!("tag_{}_{}", input.tenant_id, tag);
        sqlx::query(
            r#"
            INSERT INTO community_tag (id, tenant_id, slug, title, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (tenant_id, slug) DO UPDATE SET title = excluded.title
            "#,
        )
        .bind(&tag_id)
        .bind(&input.tenant_id)
        .bind(&tag)
        .bind(&tag)
        .bind(&input.now)
        .execute(&mut **tx)
        .await?;
        sqlx::query(
            "INSERT INTO community_entry_tag (entry_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(&input.id)
        .bind(&tag_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

fn optional_string_cell(row: &sqlx::postgres::PgRow, column: &str) -> Option<String> {
    row.try_get::<Option<String>, _>(column).ok().flatten()
}

fn string_cell(row: &sqlx::postgres::PgRow, column: &str) -> String {
    optional_string_cell(row, column).unwrap_or_default()
}

fn integer_cell(row: &sqlx::postgres::PgRow, column: &str) -> i64 {
    row.try_get::<i64, _>(column)
        .or_else(|_| row.try_get::<i32, _>(column).map(i64::from))
        .unwrap_or(0)
}

fn bool_cell(row: &sqlx::postgres::PgRow, column: &str) -> bool {
    row.try_get::<bool, _>(column)
        .or_else(|_| row.try_get::<i64, _>(column).map(|value| value != 0))
        .unwrap_or(false)
}

fn optional_integer_cell(row: &sqlx::postgres::PgRow, column: &str) -> Option<i64> {
    row.try_get::<Option<i64>, _>(column)
        .ok()
        .flatten()
        .or_else(|| {
            row.try_get::<Option<i32>, _>(column)
                .ok()
                .flatten()
                .map(i64::from)
        })
}

fn optional_f64_cell(row: &sqlx::postgres::PgRow, column: &str) -> Option<f64> {
    row.try_get::<Option<f64>, _>(column).ok().flatten()
}

fn text_array_cell(row: &sqlx::postgres::PgRow, column: &str) -> Vec<String> {
    row.try_get::<Vec<String>, _>(column).unwrap_or_default()
}

fn qr_codes_cell(row: &sqlx::postgres::PgRow, column: &str) -> Vec<CommunityStoredGroupQr> {
    row.try_get::<Option<sqlx::types::Json<Vec<CommunityStoredGroupQr>>>, _>(column)
        .ok()
        .flatten()
        .map(|json| json.0)
        .unwrap_or_default()
}

pub async fn list_members(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
) -> Result<Vec<CommunityStoredMember>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, tenant_id, category_id, user_id, user_name, role, status, bio,
               tier_id, tier_name, membership_expires_at, agent_level, last_order_id, joined_at
        FROM community_member
        WHERE tenant_id = $1 AND category_id = $2
        ORDER BY CASE role WHEN 'owner' THEN 0 WHEN 'admin' THEN 1 ELSE 2 END, joined_at ASC
        "#,
    )
    .bind(tenant_id)
    .bind(category_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .iter()
        .map(|row| CommunityStoredMember {
            id: string_cell(row, "id"),
            tenant_id: string_cell(row, "tenant_id"),
            category_id: string_cell(row, "category_id"),
            user_id: string_cell(row, "user_id"),
            user_name: string_cell(row, "user_name"),
            role: string_cell(row, "role"),
            status: string_cell(row, "status"),
            bio: optional_string_cell(row, "bio"),
            tier_id: optional_string_cell(row, "tier_id"),
            tier_name: optional_string_cell(row, "tier_name"),
            membership_expires_at: optional_string_cell(row, "membership_expires_at"),
            agent_level: optional_string_cell(row, "agent_level"),
            last_order_id: optional_string_cell(row, "last_order_id"),
            joined_at: string_cell(row, "joined_at"),
        })
        .collect())
}

pub async fn current_member(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    user_id: &str,
) -> Result<Option<CommunityStoredMember>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT id, tenant_id, category_id, user_id, user_name, role, status, bio,
               tier_id, tier_name, membership_expires_at, agent_level, last_order_id, joined_at
        FROM community_member
        WHERE tenant_id = $1 AND category_id = $2 AND user_id = $3
        "#,
    )
    .bind(tenant_id)
    .bind(category_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|row| CommunityStoredMember {
        id: string_cell(&row, "id"),
        tenant_id: string_cell(&row, "tenant_id"),
        category_id: string_cell(&row, "category_id"),
        user_id: string_cell(&row, "user_id"),
        user_name: string_cell(&row, "user_name"),
        role: string_cell(&row, "role"),
        status: string_cell(&row, "status"),
        bio: optional_string_cell(&row, "bio"),
        tier_id: optional_string_cell(&row, "tier_id"),
        tier_name: optional_string_cell(&row, "tier_name"),
        membership_expires_at: optional_string_cell(&row, "membership_expires_at"),
        agent_level: optional_string_cell(&row, "agent_level"),
        last_order_id: optional_string_cell(&row, "last_order_id"),
        joined_at: string_cell(&row, "joined_at"),
    }))
}

pub async fn create_member(pool: &PgPool, input: NewCommunityMember) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO community_member
            (id, tenant_id, category_id, user_id, user_name, role, status, bio, joined_at, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, 'active', $7, $8, $9, $10)
        ON CONFLICT (tenant_id, category_id, user_id) DO NOTHING
        "#,
    )
    .bind(input.id)
    .bind(input.tenant_id)
    .bind(input.category_id)
    .bind(input.user_id)
    .bind(input.user_name)
    .bind(input.role)
    .bind(input.bio)
    .bind(&input.now)
    .bind(&input.now)
    .bind(&input.now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_member(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    member_id: &str,
    patch: &CommunityMemberPatch,
) -> Result<(), sqlx::Error> {
    let existing = list_members(pool, tenant_id, category_id).await?;
    let Some(existing) = existing.into_iter().find(|member| member.id == member_id) else {
        return Ok(());
    };
    sqlx::query(
        r#"
        UPDATE community_member
        SET role = $1, status = $2, tier_id = $3, tier_name = $4,
            membership_expires_at = $5, agent_level = $6, last_order_id = $7, updated_at = $8
        WHERE tenant_id = $9 AND category_id = $10 AND id = $11
        "#,
    )
    .bind(patch.role.as_ref().unwrap_or(&existing.role))
    .bind(patch.status.as_ref().unwrap_or(&existing.status))
    .bind(patch.tier_id.as_ref().or(existing.tier_id.as_ref()))
    .bind(patch.tier_name.as_ref().or(existing.tier_name.as_ref()))
    .bind(
        patch
            .membership_expires_at
            .as_ref()
            .or(existing.membership_expires_at.as_ref()),
    )
    .bind(patch.agent_level.as_ref().or(existing.agent_level.as_ref()))
    .bind(
        patch
            .last_order_id
            .as_ref()
            .or(existing.last_order_id.as_ref()),
    )
    .bind(chrono::Utc::now().to_rfc3339())
    .bind(tenant_id)
    .bind(category_id)
    .bind(member_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_member(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    member_id: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM community_member WHERE tenant_id = $1 AND category_id = $2 AND id = $3",
    )
    .bind(tenant_id)
    .bind(category_id)
    .bind(member_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn list_groups(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
) -> Result<Vec<CommunityStoredGroup>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, tenant_id, category_id, name, platform, description, member_count, qr_codes, created_at, updated_at
        FROM community_group
        WHERE tenant_id = $1 AND category_id = $2
        ORDER BY created_at ASC
        "#,
    )
    .bind(tenant_id)
    .bind(category_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .iter()
        .map(|row| CommunityStoredGroup {
            id: string_cell(row, "id"),
            tenant_id: string_cell(row, "tenant_id"),
            category_id: string_cell(row, "category_id"),
            name: string_cell(row, "name"),
            platform: string_cell(row, "platform"),
            description: optional_string_cell(row, "description"),
            member_count: integer_cell(row, "member_count"),
            qr_codes: qr_codes_cell(row, "qr_codes"),
            created_at: string_cell(row, "created_at"),
            updated_at: string_cell(row, "updated_at"),
        })
        .collect())
}

pub async fn create_group(pool: &PgPool, input: NewCommunityGroup) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO community_group
            (id, tenant_id, category_id, name, platform, description, member_count, qr_codes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(input.id)
    .bind(input.tenant_id)
    .bind(input.category_id)
    .bind(input.name)
    .bind(input.platform)
    .bind(input.description)
    .bind(input.member_count)
    .bind(sqlx::types::Json(&input.qr_codes))
    .bind(&input.now)
    .bind(&input.now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_group(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    group_id: &str,
    patch: &CommunityGroupPatch,
) -> Result<(), sqlx::Error> {
    let existing = list_groups(pool, tenant_id, category_id).await?;
    let Some(existing) = existing.into_iter().find(|group| group.id == group_id) else {
        return Ok(());
    };
    sqlx::query(
        r#"
        UPDATE community_group
        SET name = $1, platform = $2, description = $3, member_count = $4, qr_codes = $5, updated_at = $6
        WHERE tenant_id = $7 AND category_id = $8 AND id = $9
        "#,
    )
    .bind(patch.name.as_ref().unwrap_or(&existing.name))
    .bind(patch.platform.as_ref().unwrap_or(&existing.platform))
    .bind(patch.description.as_ref().or(existing.description.as_ref()))
    .bind(patch.member_count.unwrap_or(existing.member_count))
    .bind(sqlx::types::Json(patch.qr_codes.as_ref().unwrap_or(&existing.qr_codes)))
    .bind(chrono::Utc::now().to_rfc3339())
    .bind(tenant_id)
    .bind(category_id)
    .bind(group_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_group(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    group_id: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM community_group WHERE tenant_id = $1 AND category_id = $2 AND id = $3",
    )
    .bind(tenant_id)
    .bind(category_id)
    .bind(group_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

fn tier_from_row(row: &sqlx::postgres::PgRow) -> CommunityStoredTier {
    CommunityStoredTier {
        id: string_cell(row, "id"),
        tenant_id: string_cell(row, "tenant_id"),
        category_id: string_cell(row, "category_id"),
        name: string_cell(row, "name"),
        description: optional_string_cell(row, "description"),
        price: optional_f64_cell(row, "price").unwrap_or(0.0),
        duration_days: integer_cell(row, "duration_days"),
        lifetime_price: optional_f64_cell(row, "lifetime_price"),
        lifetime_package_id: optional_string_cell(row, "lifetime_package_id"),
        benefits: text_array_cell(row, "benefits"),
        agent_level: optional_string_cell(row, "agent_level"),
        catalog_package_id: optional_string_cell(row, "catalog_package_id"),
        sort_order: integer_cell(row, "sort_order"),
        enabled: bool_cell(row, "enabled"),
    }
}

pub async fn list_tiers(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    enabled_only: bool,
) -> Result<Vec<CommunityStoredTier>, sqlx::Error> {
    let rows = if enabled_only {
        sqlx::query(
            r#"
            SELECT id, tenant_id, category_id, name, description, price::float8,
                   duration_days, lifetime_price::float8, lifetime_package_id, benefits,
                   agent_level, catalog_package_id, sort_order, enabled
            FROM community_membership_tier
            WHERE tenant_id = $1 AND category_id = $2 AND enabled = TRUE
            ORDER BY sort_order ASC, created_at ASC
            "#,
        )
        .bind(tenant_id)
        .bind(category_id)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT id, tenant_id, category_id, name, description, price::float8,
                   duration_days, lifetime_price::float8, lifetime_package_id, benefits,
                   agent_level, catalog_package_id, sort_order, enabled
            FROM community_membership_tier
            WHERE tenant_id = $1 AND category_id = $2
            ORDER BY sort_order ASC, created_at ASC
            "#,
        )
        .bind(tenant_id)
        .bind(category_id)
        .fetch_all(pool)
        .await?
    };
    Ok(rows.iter().map(tier_from_row).collect())
}

/// Lists membership tiers for many circles with one set-based query.
///
/// `category_ids` is bound once to `category_id = ANY($2::text[])`, scoped by
/// `tenant_id = $1` (the community schema's only isolation boundary; circles
/// and tiers carry no organization dimension). An empty `category_ids` returns
/// an empty map *without issuing a query*: `ANY('{}')` would still be a
/// pointless round trip.
///
/// Rows are ordered by `category_id ASC, sort_order ASC, created_at ASC`, so
/// each category bucket keeps exactly the per-category ordering of
/// [`list_tiers`] (`sort_order` first, then `created_at`); the in-memory
/// grouping below preserves that row order inside every bucket.
pub async fn list_tiers_for_categories(
    pool: &PgPool,
    tenant_id: &str,
    category_ids: &[String],
) -> Result<HashMap<String, Vec<CommunityStoredTier>>, sqlx::Error> {
    if category_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let rows = sqlx::query(
        r#"
        SELECT id, tenant_id, category_id, name, description, price::float8,
               duration_days, lifetime_price::float8, lifetime_package_id, benefits,
               agent_level, catalog_package_id, sort_order, enabled
        FROM community_membership_tier
        WHERE tenant_id = $1 AND category_id = ANY($2::text[])
        ORDER BY category_id ASC, sort_order ASC, created_at ASC
        "#,
    )
    .bind(tenant_id)
    .bind(category_ids.to_vec())
    .fetch_all(pool)
    .await?;
    Ok(group_tiers_by_category(rows.iter().map(tier_from_row)))
}

/// Buckets already-ordered tiers by `category_id`, preserving their order.
fn group_tiers_by_category(
    tiers: impl IntoIterator<Item = CommunityStoredTier>,
) -> HashMap<String, Vec<CommunityStoredTier>> {
    let mut grouped: HashMap<String, Vec<CommunityStoredTier>> = HashMap::new();
    for tier in tiers {
        grouped
            .entry(tier.category_id.clone())
            .or_default()
            .push(tier);
    }
    grouped
}

pub async fn accumulate_category_revenue(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    amount: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE community_category
        SET revenue_raised = revenue_raised + $3::numeric, updated_at = $4
        WHERE tenant_id = $1 AND id = $2
        "#,
    )
    .bind(tenant_id)
    .bind(category_id)
    .bind(amount)
    .bind(chrono::Utc::now().to_rfc3339())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn create_tier(pool: &PgPool, input: NewCommunityTier) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO community_membership_tier
            (id, tenant_id, category_id, name, description, price, duration_days,
             lifetime_price, benefits, agent_level, catalog_package_id, sort_order, enabled, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6::numeric, $7, $8::numeric, $9::jsonb, $10, $11, $12, FALSE, $13, $14)
        "#,
    )
    .bind(input.id)
    .bind(input.tenant_id)
    .bind(input.category_id)
    .bind(input.name)
    .bind(input.description)
    .bind(input.price)
    .bind(input.duration_days)
    .bind(input.lifetime_price)
    .bind(serde_json::to_string(&input.benefits).unwrap_or_else(|_| "[]".to_owned()))
    .bind(input.agent_level.as_deref())
    .bind(Option::<String>::None)
    .bind(input.sort_order)
    .bind(&input.now)
    .bind(&input.now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_tier(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    tier_id: &str,
    patch: &CommunityTierPatch,
) -> Result<(), sqlx::Error> {
    let existing = list_tiers(pool, tenant_id, category_id, false).await?;
    let Some(existing) = existing.into_iter().find(|tier| tier.id == tier_id) else {
        return Ok(());
    };
    sqlx::query(
        r#"
        UPDATE community_membership_tier
        SET name = $1, description = $2, price = $3::numeric, duration_days = $4,
            lifetime_price = $5::numeric, lifetime_package_id = $6,
            benefits = $7::jsonb, agent_level = $8, catalog_package_id = $9,
            sort_order = $10, enabled = $11, updated_at = $12
        WHERE tenant_id = $13 AND category_id = $14 AND id = $15
        "#,
    )
    .bind(patch.name.as_ref().unwrap_or(&existing.name))
    .bind(patch.description.as_ref().or(existing.description.as_ref()))
    .bind(patch.price.unwrap_or(existing.price))
    .bind(patch.duration_days.unwrap_or(existing.duration_days))
    .bind(patch.lifetime_price.or(existing.lifetime_price))
    .bind(
        patch
            .lifetime_package_id
            .as_ref()
            .or(existing.lifetime_package_id.as_ref()),
    )
    .bind(
        serde_json::to_string(patch.benefits.as_ref().unwrap_or(&existing.benefits))
            .unwrap_or_else(|_| "[]".to_owned()),
    )
    .bind(patch.agent_level.as_ref().or(existing.agent_level.as_ref()))
    .bind(
        patch
            .catalog_package_id
            .as_ref()
            .or(existing.catalog_package_id.as_ref()),
    )
    .bind(patch.sort_order.unwrap_or(existing.sort_order))
    .bind(patch.enabled.unwrap_or(existing.enabled))
    .bind(chrono::Utc::now().to_rfc3339())
    .bind(tenant_id)
    .bind(category_id)
    .bind(tier_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_tier(
    pool: &PgPool,
    tenant_id: &str,
    category_id: &str,
    tier_id: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM community_membership_tier WHERE tenant_id = $1 AND category_id = $2 AND id = $3",
    )
    .bind(tenant_id)
    .bind(category_id)
    .bind(tier_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored_tier(category_id: &str, tier_id: &str, sort_order: i64) -> CommunityStoredTier {
        CommunityStoredTier {
            id: tier_id.to_owned(),
            tenant_id: "100001".to_owned(),
            category_id: category_id.to_owned(),
            name: tier_id.to_owned(),
            description: None,
            price: 1.0,
            duration_days: 365,
            lifetime_price: None,
            lifetime_package_id: None,
            benefits: Vec::new(),
            agent_level: None,
            catalog_package_id: None,
            sort_order,
            enabled: false,
        }
    }

    #[test]
    fn grouping_maps_each_category_to_its_own_tiers_in_order() {
        let grouped = group_tiers_by_category(vec![
            stored_tier("category-a", "a-first", 1),
            stored_tier("category-a", "a-second", 2),
            stored_tier("category-b", "b-first", 1),
        ]);

        assert_eq!(grouped.len(), 2);
        assert_eq!(
            grouped["category-a"]
                .iter()
                .map(|tier| tier.id.as_str())
                .collect::<Vec<_>>(),
            vec!["a-first", "a-second"],
        );
        assert_eq!(
            grouped["category-b"]
                .iter()
                .map(|tier| tier.id.as_str())
                .collect::<Vec<_>>(),
            vec!["b-first"],
        );
    }

    #[tokio::test]
    async fn an_empty_category_list_issues_no_tier_query() {
        // Lazy pool to an unreachable server: the empty-category early return
        // must never try to connect, so `ANY('{}')` is never sent.
        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(250))
            .connect_lazy("postgres://community:community@127.0.0.1:1/community")
            .expect("build lazy pool");
        let tiers = list_tiers_for_categories(&pool, "100001", &[])
            .await
            .expect("empty category list must not query");
        assert!(tiers.is_empty());
        pool.close().await;
    }
}
