//! GitOps Controller binary for managing Multipass resurrection nodes.

use axum::{routing::get, Router};
use clap::Parser;
use futures::StreamExt;
use kube::api::Api;
use kube::Client;
use kube_runtime::watcher;
use multipass_controller::crd::MultipassNode;
use multipass_controller::reconciler::{error_policy, reconcile, ContextData};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about = "Cylon Multipass VM GitOps Controller")]
struct Args {
    /// Port for health check server
    #[arg(short, long, env = "CMC_PORT", default_value_t = 8085)]
    port: u16,

    /// Path to base cloud-init template
    #[arg(
        long,
        env = "CMC_CLOUD_INIT_TEMPLATE",
        default_value = "/home/casibbald/Workspace/microscaler/cylon-images/multipass/cloud-init.yaml"
    )]
    cloud_init_template: String,

    /// Path to regenesis-agent script
    #[arg(
        long,
        env = "CMC_REGENESIS_AGENT",
        default_value = "/home/casibbald/Workspace/microscaler/cylon-regenesis/scripts/regenesis-agent"
    )]
    regenesis_agent: String,

    /// Path to cylon-host.service unit
    #[arg(
        long,
        env = "CMC_CYLON_HOST_SERVICE",
        default_value = "/home/casibbald/Workspace/microscaler/cylon-regenesis/systemd/cylon-host.service"
    )]
    cylon_host_service: String,

    /// Path to regenesis-agent.service unit
    #[arg(
        long,
        env = "CMC_REGENESIS_AGENT_SERVICE",
        default_value = "/home/casibbald/Workspace/microscaler/cylon-regenesis/systemd/regenesis-agent.service"
    )]
    regenesis_agent_service: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let args = Args::parse();
    info!("Starting cylon-multipass-controller...");
    info!("Configuration: {:?}", args);

    // 2. Initialize Kubernetes client
    let client = Client::try_default().await?;

    // 3. Create context shared by reconciler turns
    let context = Arc::new(ContextData {
        client: client.clone(),
        cloud_init_template_path: args.cloud_init_template,
        regenesis_agent_path: args.regenesis_agent,
        cylon_host_service_path: args.cylon_host_service,
        regenesis_agent_service_path: args.regenesis_agent_service,
    });

    // 4. Start health probe server in the background
    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route("/readyz", get(|| async { "ok" }));
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    info!("Health probe server listening on {}", addr);
    
    tokio::spawn(async move {
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                if let Err(e) = axum::serve(listener, app).await {
                    error!("Axum server error: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to bind to health check port: {}", e);
            }
        }
    });

    // 5. Watch MultipassNode resources in all namespaces
    let api = Api::<MultipassNode>::all(client);
    let watcher_config = watcher::Config::default().any_semantic();

    info!("Starting watcher stream for MultipassNode resources...");
    
    kube_runtime::Controller::new(api, watcher_config)
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok((o, _)) => info!("Reconciled resource: {:?}", o),
                Err(e) => error!("Reconciliation error: {}", e),
            }
        })
        .await;

    Ok(())
}
