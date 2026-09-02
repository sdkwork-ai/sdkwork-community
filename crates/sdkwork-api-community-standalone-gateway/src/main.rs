mod redis_config;
mod runtime_config;

use runtime_config::GatewayRuntimeConfig;
use sdkwork_api_community_assembly::assemble_api_router_runtime;
use sdkwork_iam_web_adapter::{
    iam_web_request_context_resolver_from_database_pool_for_audiences, IamAuditEmitter,
    IamSecurityEventEmitter,
};
use sdkwork_web_bootstrap::{
    init_tracing_from_env, shared_concurrent_admission_store, shared_idempotency_store,
    shared_rate_limit_store, ApiModuleRegistry, ComposedApiAssembly, CompositeReadinessCheck,
    ReadinessCheck, RedisReadinessCheck,
};
use std::sync::Arc;

const APPLICATION_ID: &str = "sdkwork-community";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    install_process_crypto_provider()?;
    init_tracing_from_env();

    let config = GatewayRuntimeConfig::from_env()?;
    let runtime = assemble_api_router_runtime().await?;
    let resolver = iam_web_request_context_resolver_from_database_pool_for_audiences(
        runtime.database_pool.clone(),
        &[APPLICATION_ID],
    )
    .await?;
    let mut module_registry = ApiModuleRegistry::new();
    module_registry.add_modules(vec![runtime.contribution]);
    let mut composed = module_registry.try_compose("SDKWork Community API")?;
    let manifest = composed.route_manifest.clone();
    let mut framework =
        sdkwork_iam_web_adapter::build_web_framework_builder(resolver, manifest, Vec::new());

    if let Some(redis) = config.redis() {
        let store_prefix = format!("{}:web", redis.key_prefix());
        framework = framework
            .rate_limit_store(shared_rate_limit_store(
                redis.url(),
                format!("{store_prefix}:rate-limit"),
            )?)
            .idempotency_store(shared_idempotency_store(
                redis.url(),
                format!("{store_prefix}:idempotency"),
            )?)
            .concurrent_admission_store(shared_concurrent_admission_store(
                redis.url(),
                format!("{store_prefix}:concurrent-admission"),
            )?);
        let redis_readiness =
            Arc::new(RedisReadinessCheck::new(redis.url())?) as Arc<dyn ReadinessCheck>;
        composed.readiness_check = Arc::new(CompositeReadinessCheck::new(vec![
            composed.readiness_check.clone(),
            redis_readiness,
        ]));
    }

    if config.is_production() {
        let postgres_pool = runtime
            .database_pool
            .as_postgres()
            .cloned()
            .ok_or("production Community gateway requires PostgreSQL")?;
        framework = framework
            .audit_emitter(Arc::new(IamAuditEmitter::new(
                postgres_pool.clone(),
                APPLICATION_ID,
                config.environment.as_str(),
            )))
            .security_event_emitter(Arc::new(IamSecurityEventEmitter::new(
                postgres_pool,
                config.environment.as_str(),
            )));
    }

    let app = composed.into_hosted(framework).router;
    let listener = tokio::net::TcpListener::bind(config.bind).await?;
    tracing::info!(
        target = "community.bootstrap",
        addr = %config.bind,
        environment = config.environment.as_str(),
        "community standalone gateway listening"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn install_process_crypto_provider() -> Result<(), String> {
    if rustls::crypto::CryptoProvider::get_default().is_some() {
        return Ok(());
    }
    rustls::crypto::ring::default_provider()
        .install_default()
        .map_err(|_| "failed to install the process-level Rustls crypto provider".to_owned())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::error!(target = "community.runtime", %error, "Ctrl+C handler failed");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(error) => {
                tracing::error!(target = "community.runtime", %error, "SIGTERM handler failed");
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_crypto_provider_before_runtime_composition() {
        install_process_crypto_provider().expect("process crypto provider");
        assert!(rustls::crypto::CryptoProvider::get_default().is_some());
    }
}
