//! The Multipass resurrection node controller library.

/// Cloud-init template renderer and merger.
pub mod cloud_init;
/// Custom Resource Definitions (CRDs) for resurrection nodes.
pub mod crd;
/// Resurrection Hub integration client.
pub mod hub;
/// Multipass CLI command wrapper and executor.
pub mod multipass;
/// The controller reconciler loop and drift detection logic.
pub mod reconciler;
