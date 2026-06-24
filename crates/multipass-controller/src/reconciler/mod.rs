//! Main reconciliation control loop for Multipass nodes.

pub mod drift;
pub mod utils;

use crate::cloud_init::generate_cloud_init;
use crate::crd::{MultipassNode, MultipassNodeStatus};
use crate::hub::drain_node;
use crate::multipass::{
    delete_vm, exec_command, get_vm_info, launch_vm, set_setting, start_vm, stop_vm, transfer_file,
};
use anyhow::{anyhow, Result};
use k8s_openapi::api::core::v1::Secret;
use kube::api::Api;
use kube::Client;
use kube_runtime::controller::Action;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

pub use drift::{detect_drift, DriftType};
pub use utils::{add_finalizer, has_finalizer, parse_memory_to_mb, remove_finalizer, update_status};

/// The name of the finalizer used to block deletion of the CRD until VM resources are cleaned up.
pub const FINALIZER_NAME: &str = "multipass.cylon.dev/finalizer";

/// Context data shared between reconciliation runs.
pub struct ContextData {
    /// Kubernetes API client.
    pub client: Client,
    /// Path to base cloud-init template on the host.
    pub cloud_init_template_path: String,
    /// Path to the regenesis-agent installer script.
    pub regenesis_agent_path: String,
    /// Path to the cylon-host systemd service file.
    pub cylon_host_service_path: String,
    /// Path to the regenesis-agent systemd service file.
    pub regenesis_agent_service_path: String,
}

/// Errors returned by the reconciler logic.
#[derive(Debug, thiserror::Error)]
pub enum ReconcilerError {
    /// Error originating from the Kubernetes client/API.
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),
    /// Catch-all reconciliation failure error.
    #[error("Reconciliation failed: {0}")]
    Failed(#[from] anyhow::Error),
}

/// The core entry point for the Multipass reconciler loop.
pub async fn reconcile(
    node: Arc<MultipassNode>,
    ctx: Arc<ContextData>,
) -> Result<Action, ReconcilerError> {
    let ns = node.metadata.namespace.as_deref().unwrap_or("default");
    let name = node
        .metadata
        .name
        .as_deref()
        .ok_or_else(|| ReconcilerError::Failed(anyhow!("Resource name is missing")))?;

    let api: Api<MultipassNode> = Api::namespaced(ctx.client.clone(), ns);

    // 1. Handle deletion first
    if node.metadata.deletion_timestamp.is_some() {
        return handle_deletion(&node, &api, &ctx).await;
    }

    // 2. Add finalizer if not present
    if !has_finalizer(&node).await {
        add_finalizer(&node, &api).await?;
        return Ok(Action::requeue(Duration::from_secs(1)));
    }

    // 3. Perform main reconciliation
    match reconcile_node(&node, &ctx).await {
        Ok((status, requeue_delay)) => {
            update_status(&node, &api, status).await?;
            Ok(Action::requeue(requeue_delay))
        }
        Err(e) => {
            error!("Error reconciling MultipassNode {}: {}", name, e);
            let mut status = node.status.clone().unwrap_or_default();
            status.ready = Some("False".to_string());
            status.lifecycle_state = Some("Failed".to_string());
            status.error = Some(e.to_string());
            let _ = update_status(&node, &api, status).await;
            Err(ReconcilerError::Failed(e))
        }
    }
}

/// Error policy handler for recovery and requeueing when a reconciliation turn fails.
pub fn error_policy(_node: Arc<MultipassNode>, error: &ReconcilerError, _ctx: Arc<ContextData>) -> Action {
    warn!("Reconciliation error policy triggered: {}", error);
    Action::requeue(Duration::from_secs(30))
}

async fn handle_deletion(
    node: &MultipassNode,
    api: &Api<MultipassNode>,
    _ctx: &ContextData,
) -> Result<Action, ReconcilerError> {
    let name = node.metadata.name.as_deref().ok_or_else(|| ReconcilerError::Failed(anyhow!("Missing resource name")))?;
    info!("Handling deletion for MultipassNode {}", name);

    // Check if VM exists
    let vm_info = get_vm_info(name).map_err(ReconcilerError::Failed)?;
    if vm_info.is_some() {
        teardown_vm(name, node, api).await?;
    }

    // Remove finalizer
    remove_finalizer(node, api).await?;
    Ok(Action::await_change())
}

async fn teardown_vm(
    name: &str,
    node: &MultipassNode,
    api: &Api<MultipassNode>,
) -> Result<(), ReconcilerError> {
    // Update status to Draining
    let mut status = node.status.clone().unwrap_or_default();
    status.lifecycle_state = Some("Draining".to_string());
    status.ready = Some("False".to_string());
    update_status(node, api, status).await?;

    // 1. Drain node in Cylon Hub
    let hub_api = &node.spec.config.hub_api;
    if let Err(e) = drain_node(hub_api, name).await {
        warn!("Failed to drain node {} on hub: {}", name, e);
    }

    // Update status to Deleting
    let mut status = node.status.clone().unwrap_or_default();
    status.lifecycle_state = Some("Deleting".to_string());
    update_status(node, api, status).await?;

    // 2. Delete and purge VM
    info!("Deleting and purging VM {}", name);
    delete_vm(name, true).map_err(ReconcilerError::Failed)?;
    Ok(())
}

async fn reconcile_node(
    node: &MultipassNode,
    ctx: &ContextData,
) -> Result<(MultipassNodeStatus, Duration)> {
    let name = node.metadata.name.as_deref().ok_or_else(|| anyhow!("Missing resource name"))?;
    let ns = node.metadata.namespace.as_deref().unwrap_or("default");

    // Fetch GitHub Token Secret
    let secret_api: Api<Secret> = Api::namespaced(ctx.client.clone(), ns);
    let gh_secret = secret_api.get(&node.spec.github_token_secret_ref.name).await?;
    let gh_data = gh_secret.data.ok_or_else(|| anyhow!("Secret has no data"))?;
    let gh_token_bytes = gh_data.get(&node.spec.github_token_secret_ref.key)
        .ok_or_else(|| anyhow!("GitHub token key not found in secret"))?;
    let github_token = String::from_utf8(gh_token_bytes.0.clone())?.trim().to_string();

    // Fetch Certificates Secret
    let certs_secret = secret_api.get(&node.spec.certs_secret_ref.name).await?;
    let certs_data = certs_secret.data.ok_or_else(|| anyhow!("Certificates secret has no data"))?;
    
    let ca_crt = String::from_utf8(certs_data.get("ca.crt").ok_or_else(|| anyhow!("ca.crt missing"))?.0.clone())?;
    let client_crt = String::from_utf8(certs_data.get("cylon-server.crt").ok_or_else(|| anyhow!("cylon-server.crt missing"))?.0.clone())?;
    let client_key = String::from_utf8(certs_data.get("cylon-server.key").ok_or_else(|| anyhow!("cylon-server.key missing"))?.0.clone())?;

    // Load base cloud-init template
    let base_template = std::fs::read_to_string(&ctx.cloud_init_template_path)?;
    let memory_mb = parse_memory_to_mb(&node.spec.memory)?;

    // Generate cloud-init and its hash
    let rendered_cloud_init = generate_cloud_init(
        &base_template,
        &github_token,
        &node.spec.release_pin,
        &ca_crt,
        &client_crt,
        &client_key,
        name,
        &node.spec.config.hub_api,
        &node.spec.config.grpc_endpoint,
        node.spec.cpus,
        memory_mb,
    )?;

    let mut hasher = Sha256::new();
    hasher.update(rendered_cloud_init.as_bytes());
    let cloud_init_hash = format!("{:x}", hasher.finalize());

    // Check VM info
    let vm_info = get_vm_info(name)?;
    let mut status = node.status.clone().unwrap_or_default();
    status.observed_generation = node.metadata.generation;

    match vm_info {
        None => provision_new_vm(name, node, ctx, &rendered_cloud_init, &cloud_init_hash, status).await,
        Some(info) => handle_drift(name, node, ctx, info, &cloud_init_hash, status, ns).await,
    }
}

async fn provision_new_vm(
    name: &str,
    node: &MultipassNode,
    ctx: &ContextData,
    rendered_cloud_init: &str,
    cloud_init_hash: &str,
    mut status: MultipassNodeStatus,
) -> Result<(MultipassNodeStatus, Duration)> {
    info!("VM {} does not exist, creating it...", name);
    status.lifecycle_state = Some("Creating".to_string());
    status.ready = Some("False".to_string());

    // Write rendered cloud-init to temp file
    let temp_file = tempfile::NamedTempFile::new()?;
    std::fs::write(temp_file.path(), rendered_cloud_init)?;
    let cloud_init_path = temp_file.path().to_str().ok_or_else(|| anyhow!("Failed to convert temp file path to string"))?;

    // Launch VM
    launch_vm(
        name,
        &node.spec.image,
        node.spec.cpus,
        &node.spec.memory,
        &node.spec.disk,
        cloud_init_path,
    )?;

    // Wait for cloud-init inside VM
    info!("Waiting for cloud-init in VM {}...", name);
    exec_command(name, &["cloud-init", "status", "--wait"])?;

    // Transfer regenesis-agent and systemd services
    transfer_file(&ctx.regenesis_agent_path, "/tmp/regenesis-agent", name)?;
    transfer_file(&ctx.cylon_host_service_path, "/tmp/cylon-host.service", name)?;
    transfer_file(&ctx.regenesis_agent_service_path, "/tmp/regenesis-agent.service", name)?;

    // Run installer steps inside VM
    exec_command(name, &["sudo", "install", "-m", "755", "/tmp/regenesis-agent", "/usr/local/bin/regenesis-agent"])?;
    exec_command(name, &["sudo", "mkdir", "-p", "/usr/local/share/regenesis", "/etc/regenesis"])?;
    exec_command(name, &["sudo", "install", "-m", "644", "/tmp/cylon-host.service", "/usr/local/share/regenesis/cylon-host.service"])?;
    exec_command(name, &["sudo", "install", "-m", "644", "/tmp/regenesis-agent.service", "/etc/systemd/system/regenesis-agent.service"])?;
    exec_command(name, &["sudo", "rm", "-f", "/tmp/regenesis-agent", "/tmp/cylon-host.service", "/tmp/regenesis-agent.service"])?;

    // Execute full provisioning
    info!("Running regenesis-agent provisioning inside VM {}...", name);
    exec_command(
        name,
        &[
            "sudo",
            "/usr/local/bin/regenesis-agent",
            "--config",
            "/etc/regenesis/config.env",
            "--provisioning",
            "full",
        ],
    )?;

    status.lifecycle_state = Some("Running".to_string());
    status.cloud_init_hash = Some(cloud_init_hash.to_string());
    status.error = None;
    
    // Retrieve fresh info to get IP address
    if let Some(fresh_info) = get_vm_info(name)? {
        status.ip_address = fresh_info.ipv4.first().cloned();
    }
    
    Ok((status, Duration::from_secs(10)))
}

async fn handle_drift(
    name: &str,
    node: &MultipassNode,
    ctx: &ContextData,
    info: crate::multipass::VmInfo,
    cloud_init_hash: &str,
    mut status: MultipassNodeStatus,
    ns: &str,
) -> Result<(MultipassNodeStatus, Duration)> {
    // Check for drift using detect_drift
    let drift = detect_drift(&node.spec, &status, &info, cloud_init_hash)?;

    status.hardware_drift = Some(match drift {
        DriftType::None => false,
        DriftType::Mutable { .. } | DriftType::Immutable { .. } => true,
    });
    status.ip_address = info.ipv4.first().cloned();

    match drift {
        DriftType::Immutable { cloud_init_drift, disk_drift } => {
            handle_immutable_drift(name, node, ctx, status, ns, cloud_init_drift, disk_drift).await
        }
        DriftType::Mutable { cpus_drift, memory_drift } => {
            handle_mutable_drift(name, node, ctx, status, ns, cpus_drift, memory_drift).await
        }
        DriftType::None => {
            handle_no_drift(name, info, status).await
        }
    }
}

async fn handle_immutable_drift(
    name: &str,
    node: &MultipassNode,
    ctx: &ContextData,
    mut status: MultipassNodeStatus,
    ns: &str,
    cloud_init_drift: bool,
    disk_drift: bool,
) -> Result<(MultipassNodeStatus, Duration)> {
    info!(
        "Immutable drift detected for node {} (cloud_init_drift={}, disk_drift={}). Recreating VM...",
        name, cloud_init_drift, disk_drift
    );

    status.lifecycle_state = Some("Draining".to_string());
    update_status(node, &Api::namespaced(ctx.client.clone(), ns), status.clone()).await?;

    // Drain node
    if let Err(e) = drain_node(&node.spec.config.hub_api, name).await {
        warn!("Failed to drain node {} on hub during recreate: {}", name, e);
    }

    status.lifecycle_state = Some("Deleting".to_string());
    update_status(node, &Api::namespaced(ctx.client.clone(), ns), status.clone()).await?;

    // Delete VM
    delete_vm(name, true)?;

    // Requeue immediately to trigger recreation
    status.lifecycle_state = Some("Creating".to_string());
    status.ready = Some("False".to_string());
    Ok((status, Duration::from_secs(1)))
}

async fn handle_mutable_drift(
    name: &str,
    node: &MultipassNode,
    ctx: &ContextData,
    mut status: MultipassNodeStatus,
    ns: &str,
    cpus_drift: bool,
    memory_drift: bool,
) -> Result<(MultipassNodeStatus, Duration)> {
    info!(
        "Mutable drift detected for node {} (cpus_drift={}, memory_drift={}). Updating in-place...",
        name, cpus_drift, memory_drift
    );

    status.lifecycle_state = Some("Draining".to_string());
    update_status(node, &Api::namespaced(ctx.client.clone(), ns), status.clone()).await?;

    // Drain node
    if let Err(e) = drain_node(&node.spec.config.hub_api, name).await {
        warn!("Failed to drain node {} on hub during hardware update: {}", name, e);
    }

    // Stop VM
    stop_vm(name)?;

    // Update settings
    if cpus_drift {
        set_setting(&format!("local.{}.cpus", name), &node.spec.cpus.to_string())?;
    }
    if memory_drift {
        let norm_memory = node.spec.memory.replace("Gi", "G").replace("Mi", "M");
        set_setting(&format!("local.{}.memory", name), &norm_memory)?;
    }

    // Start VM
    start_vm(name)?;
    status.lifecycle_state = Some("Running".to_string());
    Ok((status, Duration::from_secs(30)))
}

async fn handle_no_drift(
    name: &str,
    info: crate::multipass::VmInfo,
    mut status: MultipassNodeStatus,
) -> Result<(MultipassNodeStatus, Duration)> {
    // If stopped, start it
    if info.state != "Running" {
        info!("VM {} is in state {}, starting it...", name, info.state);
        start_vm(name)?;
    }

    // Verify health check of Cylon host inside VM
    let health_check_cmd = ["curl", "-sf", "http://127.0.0.1:8080/health"];
    let ready = match exec_command(name, &health_check_cmd) {
        Ok(_) => "True".to_string(),
        Err(e) => {
            warn!("Health check failed for node {}: {}", name, e);
            "False".to_string()
        }
    };

    status.ready = Some(ready);
    status.error = None;
    Ok((status, Duration::from_secs(30)))
}
