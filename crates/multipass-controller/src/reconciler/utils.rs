//! Utility and helper functions for the Multipass reconciler.

use crate::crd::{MultipassNode, MultipassNodeStatus};
use crate::reconciler::{ReconcilerError, FINALIZER_NAME};
use anyhow::{anyhow, Result};
use kube::api::{Api, Patch, PatchParams};
use tracing::info;

/// Parses size strings (e.g., "16Gi", "40G") to raw bytes count.
/// Supports G, GB, Gi, GiB, M, MB, Mi, MiB.
pub fn parse_size_to_bytes(s: &str) -> Result<u64> {
    let s = s.trim().to_lowercase();
    let val_str: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    if val_str.is_empty() {
        return Err(anyhow!("No digits found in size string: {}", s));
    }
    let val: u64 = val_str.parse()?;
    let unit = s.trim_start_matches(&val_str);
    match unit {
        "g" | "gb" => Ok(val * 1_000_000_000),
        "gi" | "gib" => Ok(val * 1024 * 1024 * 1024),
        "m" | "mb" => Ok(val * 1_000_000),
        "mi" | "mib" => Ok(val * 1024 * 1024),
        _ => Err(anyhow!("Unknown unit in size string: {}", unit)),
    }
}

/// Parses memory allocation strings to Megabytes (MB).
pub fn parse_memory_to_mb(s: &str) -> Result<u64> {
    let bytes = parse_size_to_bytes(s)?;
    Ok(bytes / (1024 * 1024))
}

/// Checks if the custom resource has the Multipass finalizer registered.
pub async fn has_finalizer(node: &MultipassNode) -> bool {
    node.metadata
        .finalizers
        .as_ref()
        .map(|f| f.contains(&FINALIZER_NAME.to_string()))
        .unwrap_or(false)
}

/// Registers the finalizer on the MultipassNode custom resource.
pub async fn add_finalizer(node: &MultipassNode, api: &Api<MultipassNode>) -> Result<(), ReconcilerError> {
    let name = node.metadata.name.as_deref().ok_or_else(|| ReconcilerError::Failed(anyhow!("Missing resource name")))?;
    info!("Adding finalizer to MultipassNode {}", name);
    let patch = serde_json::json!({
        "metadata": {
            "finalizers": [FINALIZER_NAME]
        }
    });
    api.patch(name, &PatchParams::default(), &Patch::Merge(&patch)).await?;
    Ok(())
}

/// Removes the finalizer from the MultipassNode custom resource.
pub async fn remove_finalizer(node: &MultipassNode, api: &Api<MultipassNode>) -> Result<(), ReconcilerError> {
    let name = node.metadata.name.as_deref().ok_or_else(|| ReconcilerError::Failed(anyhow!("Missing resource name")))?;
    info!("Removing finalizer from MultipassNode {}", name);
    let mut finalizers = node.metadata.finalizers.clone().unwrap_or_default();
    finalizers.retain(|f| f != FINALIZER_NAME);
    let patch = serde_json::json!({
        "metadata": {
            "finalizers": finalizers
        }
    });
    api.patch(name, &PatchParams::default(), &Patch::Merge(&patch)).await?;
    Ok(())
}

/// Updates the status subresource of a MultipassNode in Kubernetes.
pub async fn update_status(
    node: &MultipassNode,
    api: &Api<MultipassNode>,
    status: MultipassNodeStatus,
) -> Result<(), ReconcilerError> {
    let name = node.metadata.name.as_deref().ok_or_else(|| ReconcilerError::Failed(anyhow!("Missing resource name")))?;
    let patch = serde_json::json!({
        "status": status
    });
    api.patch_status(name, &PatchParams::default(), &Patch::Merge(&patch)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_parse_size_to_bytes() {
        assert_eq!(parse_size_to_bytes("16Gi").unwrap(), 16_u64 * 1024 * 1024 * 1024);
        assert_eq!(parse_size_to_bytes("8G").unwrap(), 8_u64 * 1_000_000_000);
        assert_eq!(parse_size_to_bytes("512Mi").unwrap(), 512 * 1024 * 1024);
        assert_eq!(parse_size_to_bytes("100M").unwrap(), 100 * 1_000_000);
        assert!(parse_size_to_bytes("invalid").is_err());
    }

    #[test]
    fn test_parse_memory_to_mb() {
        assert_eq!(parse_memory_to_mb("16Gi").unwrap(), 16384);
        assert_eq!(parse_memory_to_mb("8G").unwrap(), 7629); // 8,000,000,000 / (1024*1024)
        assert_eq!(parse_memory_to_mb("512Mi").unwrap(), 512);
    }

    #[test]
    fn test_has_finalizer() {
        let mut node = MultipassNode::new("test-node", crate::crd::MultipassNodeSpec {
            cpus: 4,
            memory: "8Gi".to_string(),
            disk: "20Gi".to_string(),
            image: "ubuntu-24.04".to_string(),
            release_pin: "latest".to_string(),
            github_token_secret_ref: crate::crd::SecretKeySelector {
                name: "token".to_string(),
                key: "key".to_string(),
            },
            certs_secret_ref: crate::crd::SecretReference {
                name: "certs".to_string(),
            },
            network: None,
            config: crate::crd::CylonNodeConfig {
                grpc_endpoint: "https://node:50052".to_string(),
                hub_api: "http://hub:14000".to_string(),
            },
        });
        
        assert!(!tokio::runtime::Runtime::new().unwrap().block_on(has_finalizer(&node)));

        node.metadata.finalizers = Some(vec![FINALIZER_NAME.to_string()]);
        assert!(tokio::runtime::Runtime::new().unwrap().block_on(has_finalizer(&node)));
    }
}