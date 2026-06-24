use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;
use tracing::{debug, info};

/// Structure representing active VM information returned by Multipass.
#[derive(Debug, Clone, Deserialize)]
pub struct VmInfo {
    /// Number of virtual CPUs assigned to the VM.
    #[serde(rename = "cpu_count")]
    pub cpu_count: String,
    /// Map of disk names to their respective disk usage/capacity info.
    pub disks: HashMap<String, DiskInfo>,
    /// Memory allocation and usage information.
    pub memory: MemoryInfo,
    /// The current state of the VM (e.g. "Running", "Stopped").
    pub state: String,
    /// List of IPv4 addresses assigned to the VM.
    pub ipv4: Vec<String>,
}

/// Disk usage and size information of the VM.
#[derive(Debug, Clone, Deserialize)]
pub struct DiskInfo {
    /// Total capacity of the disk in bytes.
    pub total: String, // String representation of bytes
    /// Total used space on the disk in bytes.
    pub used: String,
}

/// Memory size and usage information of the VM.
#[derive(Debug, Clone, Deserialize)]
pub struct MemoryInfo {
    /// Total memory allocated to the VM in bytes.
    pub total: u64, // Bytes as number
    /// Memory currently used by the VM in bytes.
    pub used: u64,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct MultipassInfoResponse {
    pub errors: Vec<String>,
    pub info: HashMap<String, VmInfo>,
}

/// Retrieve the active VM information for a named VM, returning `None` if the VM does not exist.
pub fn get_vm_info(name: &str) -> Result<Option<VmInfo>> {
    let output = Command::new("multipass")
        .args(["info", name, "--format", "json"])
        .output()?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        if err_msg.contains("does not exist") {
            return Ok(None);
        }
        return Err(anyhow!("multipass info failed: {}", err_msg));
    }

    parse_multipass_info(&output.stdout, name)
}

fn parse_multipass_info(stdout: &[u8], name: &str) -> Result<Option<VmInfo>> {
    let response: MultipassInfoResponse = serde_json::from_slice(stdout)?;
    Ok(response.info.get(name).cloned())
}

/// Launch a new Multipass VM.
pub fn launch_vm(
    name: &str,
    image: &str,
    cpus: u32,
    memory: &str,
    disk: &str,
    cloud_init_path: &str,
) -> Result<()> {
    info!("Launching Multipass VM: {} (cpus={}, memory={}, disk={}, image={})", name, cpus, memory, disk, image);
    
    // Normalize memory and disk (Multipass wants suffix like 'G' or 'M' instead of 'Gi' or 'Mi')
    let norm_memory = memory.replace("Gi", "G").replace("Mi", "M");
    let norm_disk = disk.replace("Gi", "G").replace("Mi", "M");

    let status = Command::new("multipass")
        .args([
            "launch",
            image,
            "--name",
            name,
            "--cpus",
            &cpus.to_string(),
            "--memory",
            &norm_memory,
            "--disk",
            &norm_disk,
            "--cloud-init",
            cloud_init_path,
        ])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to launch Multipass VM: {}", name));
    }
    Ok(())
}

/// Stop a Multipass VM.
pub fn stop_vm(name: &str) -> Result<()> {
    info!("Stopping Multipass VM: {}", name);
    let status = Command::new("multipass")
        .args(["stop", name])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to stop Multipass VM: {}", name));
    }
    Ok(())
}

/// Start a Multipass VM.
pub fn start_vm(name: &str) -> Result<()> {
    info!("Starting Multipass VM: {}", name);
    let status = Command::new("multipass")
        .args(["start", name])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to start Multipass VM: {}", name));
    }
    Ok(())
}

/// Delete and optionally purge a Multipass VM.
pub fn delete_vm(name: &str, purge: bool) -> Result<()> {
    info!("Deleting Multipass VM: {} (purge={})", name, purge);
    let mut args = vec!["delete", name];
    if purge {
        args.push("--purge");
    }
    let status = Command::new("multipass")
        .args(args)
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to delete Multipass VM: {}", name));
    }
    Ok(())
}

/// Set a configuration setting in Multipass (e.g. local.<name>.cpus=6).
pub fn set_setting(key: &str, value: &str) -> Result<()> {
    info!("Setting Multipass config: {}={}", key, value);
    let status = Command::new("multipass")
        .args(["set", &format!("{}={}", key, value)])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to set Multipass config: {}={}", key, value));
    }
    Ok(())
}

/// Transfer a file from the host to the VM.
pub fn transfer_file(src: &str, dest_in_vm: &str, vm: &str) -> Result<()> {
    debug!("Transferring file to VM {}: {} -> {}", vm, src, dest_in_vm);
    let status = Command::new("multipass")
        .args(["transfer", src, &format!("{}:{}", vm, dest_in_vm)])
        .status()?;

    if !status.success() {
        return Err(anyhow!("Failed to transfer file {} to VM {}", src, vm));
    }
    Ok(())
}

/// Execute a command in the VM.
pub fn exec_command(vm: &str, cmd: &[&str]) -> Result<String> {
    debug!("Executing command inside VM {}: {:?}", vm, cmd);
    let mut args = vec!["exec", vm, "--"];
    args.extend(cmd);
    
    let output = Command::new("multipass")
        .args(args)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to execute command inside VM {}: {}",
            vm,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_parse_multipass_info_success() {
        let raw_json = r#"{
    "errors": [],
    "info": {
        "resurrection-node-1": {
            "cpu_count": "6",
            "disks": {
                "sda1": {
                    "total": "41516494336",
                    "used": "15234885120"
                }
            },
            "image_hash": "53fdde898feed8b027d94baa9cfe8229867f330a1d9c49dc7d84465ee7f229f7",
            "image_release": "24.04 LTS",
            "ipv4": [
                "10.177.76.108"
            ],
            "load": [0.09, 0.09, 0.14],
            "memory": {
                "total": 16763326464,
                "used": 815968256
            },
            "mounts": {},
            "release": "Ubuntu 24.04.4 LTS",
            "snapshot_count": "0",
            "state": "Running"
        }
    }
}"#;

        let info = parse_multipass_info(raw_json.as_bytes(), "resurrection-node-1")
            .unwrap()
            .unwrap();

        assert_eq!(info.cpu_count, "6");
        assert_eq!(info.state, "Running");
        assert_eq!(info.memory.total, 16763326464);
        assert_eq!(info.ipv4.first().unwrap(), "10.177.76.108");
        assert_eq!(info.disks.get("sda1").unwrap().total, "41516494336");
    }

    #[test]
    fn test_parse_multipass_info_missing() {
        let raw_json = r#"{
    "errors": [],
    "info": {}
}"#;
        let info = parse_multipass_info(raw_json.as_bytes(), "resurrection-node-1").unwrap();
        assert!(info.is_none());
    }

    #[test]
    fn test_stop_vm_fails_for_non_existent() {
        let res = stop_vm("non-existent-test-vm-12345");
        assert!(res.is_err());
    }

    #[test]
    fn test_start_vm_fails_for_non_existent() {
        let res = start_vm("non-existent-test-vm-12345");
        assert!(res.is_err());
    }

    #[test]
    fn test_delete_vm_fails_for_non_existent() {
        let res = delete_vm("non-existent-test-vm-12345", false);
        assert!(res.is_err());
    }

    #[test]
    fn test_set_setting_fails_for_non_existent() {
        let res = set_setting("local.non-existent-test-vm-12345.cpus", "2");
        assert!(res.is_err());
    }

    #[test]
    fn test_transfer_file_fails_for_non_existent() {
        let res = transfer_file("/dev/null", "/tmp/test", "non-existent-test-vm-12345");
        assert!(res.is_err());
    }

    #[test]
    fn test_exec_command_fails_for_non_existent() {
        let res = exec_command("non-existent-test-vm-12345", &["ls"]);
        assert!(res.is_err());
    }

    #[test]
    fn test_launch_vm_fails_for_invalid_image() {
        let res = launch_vm("non-existent-vm", "invalid-image-12345", 1, "512M", "5G", "/dev/null");
        assert!(res.is_err());
    }
}
