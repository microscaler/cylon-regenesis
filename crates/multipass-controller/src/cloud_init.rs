use anyhow::{anyhow, Result};
use serde_yaml::Value;

/// Render cloud-init configuration by merging the base template with certificates,
/// tokens, and node-specific configurations.
#[allow(clippy::too_many_arguments)]
pub fn generate_cloud_init(
    base_template_content: &str,
    github_token: &str,
    release_pin: &str,
    ca_crt: &str,
    client_crt: &str,
    client_key: &str,
    node_id: &str,
    hub_api: &str,
    grpc_endpoint: &str,
    cpus: u32,
    memory_mb: u64,
) -> Result<String> {
    let mut yaml_val: Value = serde_yaml::from_str(base_template_content)?;
    
    let mapping = yaml_val.as_mapping_mut()
        .ok_or_else(|| anyhow!("Cloud-init template is not a YAML mapping"))?;
    
    let write_files_key = Value::String("write_files".to_string());
    if !mapping.contains_key(&write_files_key) {
        mapping.insert(write_files_key.clone(), Value::Sequence(Vec::new()));
    }
    
    let write_files = mapping.get_mut(&write_files_key)
        .and_then(|v| v.as_sequence_mut())
        .ok_or_else(|| anyhow!("write_files is not a sequence"))?;
    
    // Helper to add or override a file in write_files
    let mut add_file = |path: &str, content: &str, permissions: &str, owner: &str| {
        // If file already exists in base template, remove it so we don't have duplicates
        write_files.retain(|val| {
            val.as_mapping()
                .and_then(|m| m.get("path"))
                .and_then(|v| v.as_str())
                .map(|p| p != path)
                .unwrap_or(true)
        });

        let mut file_map = serde_yaml::Mapping::new();
        file_map.insert(Value::String("path".to_string()), Value::String(path.to_string()));
        file_map.insert(Value::String("permissions".to_string()), Value::String(permissions.to_string()));
        file_map.insert(Value::String("owner".to_string()), Value::String(owner.to_string()));
        file_map.insert(Value::String("content".to_string()), Value::String(content.to_string()));
        write_files.push(Value::Mapping(file_map));
    };

    // Inject github-token
    add_file("/etc/cylon/github-token", github_token, "0600", "root:root");
    
    // Inject release-pin
    add_file("/etc/cylon/release-pin", release_pin, "0644", "root:root");
    
    // Inject certificates
    add_file("/etc/cylon/certs/ca.crt", ca_crt, "0644", "root:root");
    add_file("/etc/cylon/certs/cylon-server.crt", client_crt, "0644", "root:root");
    add_file("/etc/cylon/certs/cylon-server.key", client_key, "0600", "root:root");
    
    // Generate config.env for regenesis-agent
    let config_env = format!(
        r#"REGENESIS_PROVISIONING=full
REGENESIS_NODE_ID={}
REGENESIS_HUB_API={}
REGENESIS_GRPC_ENDPOINT={}
REGENESIS_RELEASE_PIN={}
REGENESIS_GITHUB_TOKEN_FILE=/etc/cylon/github-token
REGENESIS_CERTS_DIR=/etc/cylon/certs
REGENESIS_MEMORY_MB={}
REGENESIS_VCPU={}
"#,
        node_id, hub_api, grpc_endpoint, release_pin, memory_mb, cpus
    );
    
    add_file("/etc/regenesis/config.env", &config_env, "0644", "root:root");
    
    let rendered = serde_yaml::to_string(&yaml_val)?;
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_generate_cloud_init() {
        let base = r#"
users:
  - default
write_files:
  - path: /etc/apt/apt.conf.d/99-force-ipv4
    content: |
      Acquire::ForceIPv4 "true";
"#;
        let rendered = generate_cloud_init(
            base,
            "my-token",
            "v0.1.0",
            "ca-data",
            "crt-data",
            "key-data",
            "my-node",
            "http://hub:14000",
            "https://node:50052",
            4,
            8192,
        ).unwrap();

        let parsed: Value = serde_yaml::from_str(&rendered).unwrap();
        let write_files = parsed.get("write_files").unwrap().as_sequence().unwrap();

        // We expect: base file, token, release-pin, ca.crt, cylon-server.crt, cylon-server.key, config.env
        assert_eq!(write_files.len(), 7);

        let token_file = write_files.iter().find(|val| {
            val.get("path").unwrap().as_str() == Some("/etc/cylon/github-token")
        }).unwrap();
        assert_eq!(token_file.get("content").unwrap().as_str(), Some("my-token"));

        let config_file = write_files.iter().find(|val| {
            val.get("path").unwrap().as_str() == Some("/etc/regenesis/config.env")
        }).unwrap();
        let config_content = config_file.get("content").unwrap().as_str().unwrap();
        assert!(config_content.contains("REGENESIS_NODE_ID=my-node"));
        assert!(config_content.contains("REGENESIS_HUB_API=http://hub:14000"));
        assert!(config_content.contains("REGENESIS_GRPC_ENDPOINT=https://node:50052"));
        assert!(config_content.contains("REGENESIS_RELEASE_PIN=v0.1.0"));
        assert!(config_content.contains("REGENESIS_MEMORY_MB=8192"));
        assert!(config_content.contains("REGENESIS_VCPU=4"));
    }
}
