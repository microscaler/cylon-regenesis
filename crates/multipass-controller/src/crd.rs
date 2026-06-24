use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Desired state of a Multipass resurrection node
#[derive(CustomResource, Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "multipass.cylon.dev",
    version = "v1alpha1",
    kind = "MultipassNode",
    plural = "multipassnodes",
    namespaced,
    status = "MultipassNodeStatus",
    printcolumn = r#"{"name":"Ready", "type":"string", "jsonPath":".status.ready"}, {"name":"IP", "type":"string", "jsonPath":".status.ipAddress"}, {"name":"State", "type":"string", "jsonPath":".status.lifecycleState"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct MultipassNodeSpec {
    /// Number of CPUs to allocate
    pub cpus: u32,
    /// Memory allocation (e.g. "16Gi", "8G")
    pub memory: String,
    /// Disk size allocation (e.g. "40Gi", "20G")
    pub disk: String,
    /// Ubuntu image name (defaults to "ubuntu-24.04")
    #[serde(default = "default_image")]
    pub image: String,
    /// Release tag pin for Cylon (defaults to "latest")
    #[serde(default = "default_release_pin")]
    pub release_pin: String,
    /// Kubernetes Secret reference for GITHUB_TOKEN
    pub github_token_secret_ref: SecretKeySelector,
    /// Kubernetes Secret reference containing CA, client cert, and client key
    pub certs_secret_ref: SecretReference,
    /// Optional network configuration (e.g. bridge device)
    pub network: Option<NetworkConfig>,
    /// Core Cylon node settings
    pub config: CylonNodeConfig,
}

/// Selector for a key in a Kubernetes Secret.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SecretKeySelector {
    /// Name of the secret
    pub name: String,
    /// Key in the secret containing the token
    pub key: String,
}

/// Selector for a Kubernetes Secret containing certificates.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SecretReference {
    /// Name of the secret containing certificates
    pub name: String,
}

/// Configuration for VM networking.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConfig {
    /// Optional bridge device name (e.g. "en0", "br0")
    pub bridge: Option<String>,
}

/// Node configuration containing Hub API and gRPC endpoints.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CylonNodeConfig {
    /// Public gRPC endpoint of the resurrection node (e.g. "https://10.177.76.2:50052")
    pub grpc_endpoint: String,
    /// Resurrection Hub API endpoint (e.g. "http://10.177.76.1:14000")
    pub hub_api: String,
}

fn default_image() -> String {
    "ubuntu-24.04".to_string()
}

fn default_release_pin() -> String {
    "latest".to_string()
}

/// Observed state of the Multipass resurrection node
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct MultipassNodeStatus {
    /// Whether the Cylon host daemon is healthy ("True", "False", "Unknown")
    pub ready: Option<String>,
    /// The dynamically assigned DHCP IP address of the Multipass VM
    pub ip_address: Option<String>,
    /// Whether the active VM hardware (CPU/memory) has drifted from the spec
    pub hardware_drift: Option<bool>,
    /// Current lifecycle stage ("Creating", "Running", "Stopped", "Draining", "Deleting", "Failed")
    pub lifecycle_state: Option<String>,
    /// Observed generation of the resource
    pub observed_generation: Option<i64>,
    /// Hash of the compiled cloud-init payload used to build the VM
    pub cloud_init_hash: Option<String>,
    /// Last reconciliation error message, if any
    pub error: Option<String>,
}
