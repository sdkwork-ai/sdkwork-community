//! In-process membership package publisher adapter
//! (`APPLICATION_GATEWAY_SPEC.md` section 2.3).
//!
//! Implements the community `MembershipPackagePublisher` required port
//! against the membership repository embedded in the same gateway process,
//! so the community commerce integration consumes the mounted membership
//! capability directly instead of looping HTTP requests back through the
//! gateway's own listener. This is the default transport; the HTTP adapter is
//! a startup-validated, explicit split-deployment opt-in only
//! (`MODULE_SPEC.md` section 4.2).
//!
//! The service subject used for admin package mutations is deployment
//! configuration (environment overrides) with development defaults; it is
//! authorized by the composition root wiring this adapter, not by re-entering
//! the gateway's HTTP authentication stack.

use std::sync::Arc;

use sdkwork_community_service::{
    CommerceScope, MembershipPackagePublishFuture, MembershipPackagePublisher,
    MembershipPackageRegistration, RegisteredMembershipPackage,
};
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_membership_repository_sqlx::shared::current_timestamp_string;
use sdkwork_membership_repository_sqlx::{
    AdminMembershipStore, AdminMembershipSubject, AppMembershipEntityIdGenerator,
    CreateAdminMembershipPackageCommand, PostgresCommerceMembershipStore,
    TimestampMembershipEntityIdGenerator,
};

/// Overrides the platform service tenant id used for embedded membership
/// admin mutations. Defaults to `1`.
pub const SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID_ENV: &str =
    "SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID";

/// Overrides the platform service organization id (`0` means tenant-level
/// scope per `SUBJECT_ID_SPEC.md`). Defaults to `0`.
pub const SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_ORGANIZATION_ID_ENV: &str =
    "SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_ORGANIZATION_ID";

/// Overrides the platform service operator id recorded on admin mutations.
/// Defaults to `1`.
pub const SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_OPERATOR_ID_ENV: &str =
    "SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_OPERATOR_ID";

/// In-process publisher backed by the embedded membership repository.
pub struct EmbeddedMembershipPackagePublisher {
    store: Arc<dyn AdminMembershipStore + Send + Sync>,
    id_generator: TimestampMembershipEntityIdGenerator,
    subject: AdminMembershipSubject,
}

impl EmbeddedMembershipPackagePublisher {
    /// Builds the publisher on the process-shared PostgreSQL pool the
    /// composition root already provides to the embedded assemblies.
    pub fn from_pool(pool: &DatabasePool) -> Result<Self, String> {
        let pg_pool = pool.as_postgres().ok_or_else(|| {
            "embedded membership commerce publisher requires a PostgreSQL pool".to_owned()
        })?;
        Ok(Self {
            store: Arc::new(PostgresCommerceMembershipStore::new(pg_pool.clone())),
            id_generator: TimestampMembershipEntityIdGenerator::default(),
            subject: service_subject_from_env(),
        })
    }

    pub fn service_subject(&self) -> AdminMembershipSubject {
        self.subject
    }
}

impl MembershipPackagePublisher for EmbeddedMembershipPackagePublisher {
    fn register_membership_package<'a>(
        &'a self,
        registration: MembershipPackageRegistration,
        scope: CommerceScope,
    ) -> MembershipPackagePublishFuture<'a> {
        Box::pin(async move {
            // Mirror the HTTP surface contract: a positive duration and an
            // uppercase currency code (the backend normalizes the rest).
            if registration.duration_days <= 0 {
                return Err("membership package durationDays must be a positive integer".to_owned());
            }
            // Scope the package to the tenant that owns the tier. The
            // configured service subject stays the fallback for callers that
            // have no tenant scope.
            let subject = scoped_subject(&scope, &self.subject);
            // Stable idempotency key: the tier's package `code` is what the
            // membership repository upserts on, so a retried publish resolves
            // to the same package row and `externalId`.
            let request_id = format!("community-embedded-{}", registration.code);
            let currency_code = registration.currency_code.trim().to_ascii_uppercase();
            let package_id = self
                .id_generator
                .generate_entity_uuid()
                .map_err(|error| format!("membership package id generation failed: {error:?}"))?;
            let item = self
                .store
                .create_admin_membership_package(CreateAdminMembershipPackageCommand {
                    subject,
                    package_id,
                    input: sdkwork_membership_repository_sqlx::AdminMembershipPackageMutation {
                        category: registration.category,
                        code: registration.code,
                        package_group_id: registration.package_group_id,
                        plan_id: registration.plan_id,
                        name: registration.name,
                        price_amount: registration.price_amount,
                        currency_code,
                        duration_days: registration.duration_days,
                        discount: registration.discount,
                        status: registration.status,
                    },
                    request_id,
                    requested_at: current_timestamp_string(),
                })
                .await
                .map_err(|error| {
                    // Same service-layer mapping as the split-deployment HTTP
                    // adapter: any membership rejection is an integration
                    // failure (`CommunityServiceError::Integration`, HTTP 500).
                    format!("embedded membership package registration failed: {error:?}")
                })?;
            Ok(RegisteredMembershipPackage {
                id: item.id,
                external_id: item.external_id,
                name: item.name,
            })
        })
    }
}

/// Applies the caller's tenant/organization scope over the configured service
/// fallback. Community tenant ids are decimal strings; a non-decimal value
/// (for example a test tenant) keeps the configured fallback instead of
/// failing the mutation.
fn scoped_subject(
    scope: &CommerceScope,
    fallback: &AdminMembershipSubject,
) -> AdminMembershipSubject {
    AdminMembershipSubject {
        tenant_id: scope
            .tenant_id()
            .and_then(|value| value.parse::<i64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(fallback.tenant_id),
        organization_id: scope
            .organization_id()
            .and_then(|value| value.parse::<i64>().ok())
            .filter(|value| *value >= 0)
            .unwrap_or(fallback.organization_id),
        operator_id: fallback.operator_id,
        operator_type: fallback.operator_type,
    }
}

fn service_subject_from_env() -> AdminMembershipSubject {
    AdminMembershipSubject {
        tenant_id: positive_env_i64(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID_ENV, 1),
        organization_id: non_negative_env_i64(
            SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_ORGANIZATION_ID_ENV,
            0,
        ),
        operator_id: positive_env_i64(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_OPERATOR_ID_ENV, 1),
        // Canonical backend operator type: service/operator account.
        operator_type: 1,
    }
}

fn positive_env_i64(key: &str, default: i64) -> i64 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.trim().parse::<i64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn non_negative_env_i64(key: &str, default: i64) -> i64 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.trim().parse::<i64>().ok())
        .filter(|value| *value >= 0)
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caller_scope_overrides_the_configured_service_subject() {
        let fallback = AdminMembershipSubject {
            tenant_id: 1,
            organization_id: 0,
            operator_id: 9,
            operator_type: 1,
        };
        let scoped = scoped_subject(
            &CommerceScope::new("100001").with_organization_id("7"),
            &fallback,
        );
        assert_eq!(scoped.tenant_id, 100001);
        assert_eq!(scoped.organization_id, 7);
        assert_eq!(scoped.operator_id, 9);
        assert_eq!(scoped.operator_type, 1);

        // A tenant the deployment cannot represent numerically keeps the
        // configured service fallback instead of failing the mutation.
        let unscoped = scoped_subject(&CommerceScope::new("tenant-alpha"), &fallback);
        assert_eq!(unscoped.tenant_id, 1);
        assert_eq!(unscoped.organization_id, 0);

        let tenant_level = scoped_subject(&CommerceScope::new("42"), &fallback);
        assert_eq!(tenant_level.tenant_id, 42);
        assert_eq!(tenant_level.organization_id, 0);
    }

    // Single env-mutating test on purpose: both scenarios share process env
    // keys, and parallel tests mutating the same key would race.
    #[test]
    fn service_subject_env_parsing() {
        // Development defaults when no overrides are set.
        std::env::remove_var(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID_ENV);
        std::env::remove_var(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_ORGANIZATION_ID_ENV);
        std::env::remove_var(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_OPERATOR_ID_ENV);
        let subject = service_subject_from_env();
        assert_eq!(subject.tenant_id, 1);
        assert_eq!(subject.organization_id, 0);
        assert_eq!(subject.operator_id, 1);
        assert_eq!(subject.operator_type, 1);

        // Env override path with validation.
        std::env::set_var(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID_ENV, "7");
        assert_eq!(
            positive_env_i64(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID_ENV, 1),
            7
        );
        std::env::set_var(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID_ENV, "-3");
        assert_eq!(
            positive_env_i64(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID_ENV, 1),
            1
        );
        std::env::set_var(
            SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID_ENV,
            "not-a-number",
        );
        assert_eq!(
            positive_env_i64(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID_ENV, 1),
            1
        );
        std::env::remove_var(SDKWORK_COMMUNITY_MEMBERSHIP_SERVICE_TENANT_ID_ENV);
    }
}
