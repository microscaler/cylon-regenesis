//! Drift detection logic for resurrection nodes.

use crate::crd::{MultipassNodeSpec, MultipassNodeStatus};
use crate::multipass::VmInfo;
use crate::reconciler::utils::parse_size_to_bytes;
use anyhow::Result;

/// Enum describing types of configuration drift detected in a Multipass VM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftType {
    /// No drift detected.
    None,
    /// Mutable hardware changes (CPUs or memory) that can be adjusted in-place.
    Mutable {
        /// CPU count has drifted from specification.
        cpus_drift: bool,
        /// Memory size has drifted from specification.
        memory_drift: bool,
    },
    /// Immutable VM settings (disk size or cloud-init) that require recreation.
    Immutable {
        /// Cloud-init config hash has drifted.
        cloud_init_drift: bool,
        /// Disk size allocation has drifted.
        disk_drift: bool,
    },
}

/// Detects drift between a `MultipassNodeSpec` / `MultipassNodeStatus` and active `VmInfo`.
pub fn detect_drift(
    spec: &MultipassNodeSpec,
    status: &MultipassNodeStatus,
    info: &VmInfo,
    new_cloud_init_hash: &str,
) -> Result<DriftType> {
    let spec_cpus_str = spec.cpus.to_string();
    let cpus_drift = info.cpu_count != spec_cpus_str;

    // Memory total from Multipass is in bytes, compare with parsed memory bytes
    let spec_memory_bytes = parse_size_to_bytes(&spec.memory)?;
    let mem_diff = (info.memory.total as i128 - spec_memory_bytes as i128).abs();
    let memory_drift = mem_diff > (spec_memory_bytes as i128 / 10);

    // Disk size drift
    let spec_disk_bytes = parse_size_to_bytes(&spec.disk)?;
    let active_disk_bytes = info.disks.get("sda1")
        .map(|d| d.total.parse::<u64>().unwrap_or(0))
        .unwrap_or(0);
    let disk_drift = (active_disk_bytes as i128 - spec_disk_bytes as i128).abs() > (spec_disk_bytes as i128 / 10);

    // Cloud init hash drift
    let cloud_init_drift = status.cloud_init_hash.as_ref() != Some(&new_cloud_init_hash.to_string());

    if cloud_init_drift || disk_drift {
        Ok(DriftType::Immutable { cloud_init_drift, disk_drift })
    } else if cpus_drift || memory_drift {
        Ok(DriftType::Mutable { cpus_drift, memory_drift })
    } else {
        Ok(DriftType::None)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_detect_drift_none() {
        let spec = MultipassNodeSpec {
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
        };
        let status = MultipassNodeStatus {
            cloud_init_hash: Some("hash123".to_string()),
            ..Default::default()
        };
        let mut disks = std::collections::HashMap::new();
        disks.insert("sda1".to_string(), crate::multipass::DiskInfo {
            total: (20_u64 * 1024 * 1024 * 1024).to_string(),
            used: "0".to_string(),
        });
        let info = VmInfo {
            cpu_count: "4".to_string(),
            disks,
            memory: crate::multipass::MemoryInfo {
                total: 8_u64 * 1024 * 1024 * 1024,
                used: 0,
            },
            state: "Running".to_string(),
            ipv4: vec![],
        };

        let drift = detect_drift(&spec, &status, &info, "hash123").unwrap();
        assert_eq!(drift, DriftType::None);
    }

    #[test]
    fn test_detect_drift_mutable() {
        let spec = MultipassNodeSpec {
            cpus: 6,
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
        };
        let status = MultipassNodeStatus {
            cloud_init_hash: Some("hash123".to_string()),
            ..Default::default()
        };
        let mut disks = std::collections::HashMap::new();
        disks.insert("sda1".to_string(), crate::multipass::DiskInfo {
            total: (20_u64 * 1024 * 1024 * 1024).to_string(),
            used: "0".to_string(),
        });
        let info = VmInfo {
            cpu_count: "4".to_string(),
            disks,
            memory: crate::multipass::MemoryInfo {
                total: 8_u64 * 1024 * 1024 * 1024,
                used: 0,
            },
            state: "Running".to_string(),
            ipv4: vec![],
        };

        let drift = detect_drift(&spec, &status, &info, "hash123").unwrap();
        assert_eq!(drift, DriftType::Mutable { cpus_drift: true, memory_drift: false });
    }

    #[test]
    fn test_detect_drift_immutable() {
        let spec = MultipassNodeSpec {
            cpus: 4,
            memory: "8Gi".to_string(),
            disk: "40Gi".to_string(),
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
        };
        let status = MultipassNodeStatus {
            cloud_init_hash: Some("hash123".to_string()),
            ..Default::default()
        };
        let mut disks = std::collections::HashMap::new();
        disks.insert("sda1".to_string(), crate::multipass::DiskInfo {
            total: (20_u64 * 1024 * 1024 * 1024).to_string(),
            used: "0".to_string(),
        });
        let info = VmInfo {
            cpu_count: "4".to_string(),
            disks,
            memory: crate::multipass::MemoryInfo {
                total: 8_u64 * 1024 * 1024 * 1024,
                used: 0,
            },
            state: "Running".to_string(),
            ipv4: vec![],
        };

        let drift = detect_drift(&spec, &status, &info, "hash123").unwrap();
        assert_eq!(drift, DriftType::Immutable { cloud_init_drift: false, disk_drift: true });
    }
}
