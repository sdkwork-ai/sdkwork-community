#[cfg(feature = "test-support")]
use sdkwork_community_storage_sqlx::{
    bootstrap_community_database, CommunityFeedQuery, CommunityModerationPatch, CommunitySqlxStore,
    NewCommunityCategory, NewCommunityEntry, PostgresTestDatabase,
};
use sdkwork_community_storage_sqlx::{
    community_database_tables, community_migration_names, community_storage_capability_manifest,
};

#[cfg(feature = "test-support")]
async fn postgres_store() -> Option<(PostgresTestDatabase, CommunitySqlxStore)> {
    let Some(database) = PostgresTestDatabase::from_env()
        .await
        .expect("create isolated PostgreSQL test database")
    else {
        eprintln!(
            "skipping Community PostgreSQL repository test; set SDKWORK_DATABASE_TEST_POSTGRES_URL"
        );
        return None;
    };
    let host = bootstrap_community_database(database.pool())
        .await
        .expect("bootstrap Community PostgreSQL schema");
    let store = CommunitySqlxStore::new(host.pool().clone());
    Some((database, store))
}

#[test]
fn community_storage_manifest_declares_complete_tables_and_migrations() {
    let manifest = community_storage_capability_manifest();
    assert_eq!(manifest.name, "sdkwork-community-storage-sqlx");
    assert_eq!(manifest.schema_version, "community.storage.v1");
    assert_eq!(community_database_tables(), manifest.tables);
    assert_eq!(community_migration_names(), manifest.migrations);
    assert_eq!(
        manifest.tables,
        vec![
            "community_category",
            "community_entry",
            "community_entry_body",
            "community_tag",
            "community_entry_tag",
            "community_comment",
            "community_reaction",
            "community_moderation_event",
            "community_recommendation_snapshot",
            "community_member",
            "community_group",
            "community_membership_tier",
            "community_schema_version",
            "community_migration_lock",
        ],
    );
    assert!(manifest
        .indexes
        .contains(&"idx_community_entry_tenant_state_activity"));
    assert!(manifest
        .indexes
        .contains(&"idx_community_entry_tenant_slug"));
    assert_eq!(
        manifest.migration_plan[0].name,
        "0001_community_baseline.sql"
    );
    assert!(manifest.migration_plan[0]
        .sql
        .contains("CREATE TABLE IF NOT EXISTS community_entry"));
}

#[test]
fn community_storage_repositories_bind_to_community_tables() {
    let manifest = community_storage_capability_manifest();
    let names = manifest
        .repository_bindings
        .iter()
        .map(|binding| binding.repository_name)
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        vec![
            "community.category.repository",
            "community.entry.repository",
            "community.comment.repository",
            "community.reaction.repository",
            "community.moderation.repository",
            "community.recommendation.repository",
            "community.member.repository",
            "community.group.repository",
            "community.membership-tier.repository",
        ],
    );
}

#[tokio::test]
#[cfg(feature = "test-support")]
async fn community_sqlx_store_migrates_creates_publishes_and_reads_feed() {
    let Some((database, store)) = postgres_store().await else {
        return;
    };

    store
        .create_category(NewCommunityCategory {
            id: "category_product".to_owned(),
            tenant_id: "100001".to_owned(),
            slug: "product".to_owned(),
            title: "Product".to_owned(),
            description: Some("Product discussions".to_owned()),
            cover_image: None,
            avatar: None,
            owner_id: None,
            member_limit: None,
            is_paid: false,
            revenue_target: None,
            price: None,
            tags: Vec::new(),
            tabs: Vec::new(),
            priority: 1,
            enabled: true,
            now: "2026-06-06T00:00:00Z".to_owned(),
        })
        .await
        .expect("create category");

    store
        .create_entry(NewCommunityEntry {
            id: "entry_sdk".to_owned(),
            tenant_id: "100001".to_owned(),
            category_id: "category_product".to_owned(),
            author_id: "user_1".to_owned(),
            author_name: "Sdkwork Team".to_owned(),
            slug: "community-sdk-release".to_owned(),
            kind: "announcement".to_owned(),
            title: "Community SDK release".to_owned(),
            excerpt: "Generated community SDKs are ready.".to_owned(),
            body_markdown: "Community SDK body".to_owned(),
            tags: vec!["release".to_owned(), "sdk".to_owned()],
            media: Vec::new(),
            now: "2026-06-06T00:01:00Z".to_owned(),
        })
        .await
        .expect("create entry");

    let approved_feed = CommunityFeedQuery {
        approved_only: true,
        ..CommunityFeedQuery::default()
    };
    assert!(store
        .list_feed("100001", &approved_feed)
        .await
        .expect("draft feed")
        .items
        .is_empty());

    // A newly created entry is submitted for moderation, so the moderation
    // queue must surface it to the operator. Without this, user-created posts
    // would be invisible to both readers and moderators.
    let moderation_queue = store
        .list_moderation_queue("100001")
        .await
        .expect("moderation queue");
    assert!(
        moderation_queue.iter().any(|entry| entry.id == "entry_sdk"),
        "a new entry must enter the moderation queue"
    );

    store
        .update_moderation(
            "100001",
            "entry_sdk",
            "moderator_1",
            &CommunityModerationPatch {
                review_state: "approved".to_owned(),
                reason: None,
            },
        )
        .await
        .expect("approve entry");

    store
        .create_entry(NewCommunityEntry {
            id: "entry_discussion".to_owned(),
            tenant_id: "100001".to_owned(),
            category_id: "category_product".to_owned(),
            author_id: "user_2".to_owned(),
            author_name: "Community User".to_owned(),
            slug: "community-discussion".to_owned(),
            kind: "discussion".to_owned(),
            title: "Community discussion".to_owned(),
            excerpt: "A second paginated feed entry.".to_owned(),
            body_markdown: "Discussion body".to_owned(),
            tags: vec!["discussion".to_owned()],
            media: Vec::new(),
            now: "2026-06-06T00:02:00Z".to_owned(),
        })
        .await
        .expect("create second entry");
    store
        .update_moderation(
            "100001",
            "entry_discussion",
            "moderator_1",
            &CommunityModerationPatch {
                review_state: "approved".to_owned(),
                reason: None,
            },
        )
        .await
        .expect("approve second entry");

    let feed = store
        .list_feed(
            "100001",
            &CommunityFeedQuery {
                category_id: Some("category_product".to_owned()),
                q: Some("sdk".to_owned()),
                approved_only: true,
                ..CommunityFeedQuery::default()
            },
        )
        .await
        .expect("feed list");
    assert_eq!(feed.total_items, 1);
    assert_eq!(feed.items.len(), 1);
    assert_eq!(feed.items[0].slug, "community-sdk-release");
    assert_eq!(feed.items[0].tags, vec!["release", "sdk"]);

    let first_page = store
        .list_feed(
            "100001",
            &CommunityFeedQuery {
                page: 1,
                page_size: 1,
                approved_only: true,
                ..CommunityFeedQuery::default()
            },
        )
        .await
        .expect("first feed page");
    let second_page = store
        .list_feed(
            "100001",
            &CommunityFeedQuery {
                page: 2,
                page_size: 1,
                approved_only: true,
                ..CommunityFeedQuery::default()
            },
        )
        .await
        .expect("second feed page");
    assert_eq!(first_page.total_items, 2);
    assert_eq!(first_page.items.len(), 1);
    assert_eq!(second_page.total_items, 2);
    assert_eq!(second_page.items.len(), 1);
    assert_ne!(first_page.items[0].id, second_page.items[0].id);
    assert_eq!(
        store
            .retrieve_entry_by_slug("100001", "community-sdk-release")
            .await
            .expect("retrieve by slug")
            .expect("approved entry")
            .body_markdown,
        "Community SDK body",
    );
    database
        .close()
        .await
        .expect("clean isolated PostgreSQL test schema");
}

#[tokio::test]
#[cfg(feature = "test-support")]
async fn community_sqlx_store_sets_and_unsets_reactions() {
    use sdkwork_community_storage_sqlx::SetCommunityReaction;

    let Some((database, store)) = postgres_store().await else {
        return;
    };

    store
        .create_category(NewCommunityCategory {
            id: "category_product".to_owned(),
            tenant_id: "100001".to_owned(),
            slug: "product".to_owned(),
            title: "Product".to_owned(),
            description: None,
            cover_image: None,
            avatar: None,
            owner_id: None,
            member_limit: None,
            is_paid: false,
            revenue_target: None,
            price: None,
            tags: Vec::new(),
            tabs: Vec::new(),
            priority: 1,
            enabled: true,
            now: "2026-06-06T00:00:00Z".to_owned(),
        })
        .await
        .expect("create category");

    store
        .create_entry(NewCommunityEntry {
            id: "entry_reaction".to_owned(),
            tenant_id: "100001".to_owned(),
            category_id: "category_product".to_owned(),
            author_id: "user_1".to_owned(),
            author_name: "User One".to_owned(),
            slug: "reaction-entry".to_owned(),
            kind: "discussion".to_owned(),
            title: "Reaction entry".to_owned(),
            excerpt: "Reaction excerpt".to_owned(),
            body_markdown: "Reaction body".to_owned(),
            tags: vec![],
            media: Vec::new(),
            now: "2026-06-06T00:02:00Z".to_owned(),
        })
        .await
        .expect("create entry");

    let liked = store
        .set_reaction(SetCommunityReaction {
            id: "reaction_1".to_owned(),
            tenant_id: "100001".to_owned(),
            entry_id: "entry_reaction".to_owned(),
            user_id: "user_2".to_owned(),
            reaction_type: "like".to_owned(),
            active: true,
            now: "2026-06-06T00:03:00Z".to_owned(),
        })
        .await
        .expect("set reaction");
    assert_eq!(liked, 1);

    let duplicate = store
        .set_reaction(SetCommunityReaction {
            id: "reaction_2".to_owned(),
            tenant_id: "100001".to_owned(),
            entry_id: "entry_reaction".to_owned(),
            user_id: "user_2".to_owned(),
            reaction_type: "like".to_owned(),
            active: true,
            now: "2026-06-06T00:04:00Z".to_owned(),
        })
        .await
        .expect("duplicate reaction");
    assert_eq!(duplicate, 1);

    let unliked = store
        .set_reaction(SetCommunityReaction {
            id: "reaction_3".to_owned(),
            tenant_id: "100001".to_owned(),
            entry_id: "entry_reaction".to_owned(),
            user_id: "user_2".to_owned(),
            reaction_type: "like".to_owned(),
            active: false,
            now: "2026-06-06T00:05:00Z".to_owned(),
        })
        .await
        .expect("unset reaction");
    assert_eq!(unliked, 0);
    database
        .close()
        .await
        .expect("clean isolated PostgreSQL test schema");
}
