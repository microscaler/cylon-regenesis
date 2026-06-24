# 🛠️ Contributing to Cylon Regenesis

This guide outlines the development workflow, codebase layout, coding standards, and local validation pipelines for engineers contributing to the `cylon-regenesis` project.

---

## 🧱 Repository Shape & Code Layout

This repository is organized as a Rust workspace to maintain strict architectural boundaries:

```
cylon-regenesis/
├── .github/workflows/
│   └── ci.yml             # Consolidated serial pipeline (checks, lints, tests, release)
├── adrs/                  # Architectural Decision Records (ADRs)
├── config/                # Environment-specific base and local configs
├── crates/
│   └── multipass-controller/  # Rust-based K8s GitOps controller
│       ├── src/
│       │   ├── crd.rs         # MultipassNode Custom Resource definitions (spec & status)
│       │   ├── cloud_init.rs  # Dynamic cloud-init profile assembler
│       │   ├── multipass.rs   # CLI wrapper for hypervisor VM management
│       │   ├── hub.rs         # Resurrection Hub HTTP client & node draining API
│       │   ├── reconciler/    # Refactored modular reconciler
│       │   │   ├── mod.rs     # Main control loop and error policies
│       │   │   ├── drift.rs   # Active configuration and hardware drift detection
│       │   │   └── utils.rs   # Size parsing, finalizers, and K8s status helpers
│       │   ├── lib.rs         # Crate library module declarations
│       │   └── main.rs        # CLI parser, Axum health server, and K8s watcher
│       └── Cargo.toml
├── docs/                  # In-depth architectural designs and planning phases
├── ipxe/                  # Standard iPXE chain scripts (DCops integration)
├── scripts/               # Host shell provisioners and regenesis installers
├── systemd/               # Systemd service units (cylon-host, regenesis-agent)
├── Cargo.toml             # Root workspace Cargo configuration with strict lints
└── VERSION                # Semver version of the regenesis components
```

### Key Source Files

* [Cargo.toml](file:///Users/casibbald/Workspace/remote/microscaler/cylon-regenesis/Cargo.toml) — Enforces global compiler and clippy lint rules.
* [crd.rs](file:///Users/casibbald/Workspace/remote/microscaler/cylon-regenesis/crates/multipass-controller/src/crd.rs) — Defines the `MultipassNodeSpec` and `MultipassNodeStatus` Kubernetes schemas.
* [cloud_init.rs](file:///Users/casibbald/Workspace/remote/microscaler/cylon-regenesis/crates/multipass-controller/src/cloud_init.rs) — Compiles and renders cloud-init VM files.
* [multipass.rs](file:///Users/casibbald/Workspace/remote/microscaler/cylon-regenesis/crates/multipass-controller/src/multipass.rs) — Wraps the local Multipass hypervisor CLI.
* [hub.rs](file:///Users/casibbald/Workspace/remote/microscaler/cylon-regenesis/crates/multipass-controller/src/hub.rs) — Interfaces with the Resurrection Hub API to register and drain nodes.
* [mod.rs](file:///Users/casibbald/Workspace/remote/microscaler/cylon-regenesis/crates/multipass-controller/src/reconciler/mod.rs) — Core controller reconciler loops and backoff retry logic.
* [drift.rs](file:///Users/casibbald/Workspace/remote/microscaler/cylon-regenesis/crates/multipass-controller/src/reconciler/drift.rs) — Performs active allocation verification to identify system drifts.
* [utils.rs](file:///Users/casibbald/Workspace/remote/microscaler/cylon-regenesis/crates/multipass-controller/src/reconciler/utils.rs) — Helpers for byte parsing, K8s statuses, and resource finalizers.

---

## 🛡️ Coding Standards & Lints

To guarantee high reliability at the infrastructure layer, this project enforces strict compiler gates configured at the workspace level:

### Rust Compiler Options
* **Enforced API Documentation:** The codebase enforces documentation for all public components with `#![warn(missing_docs)]`.

### Forbidden Code Patterns
To prevent unexpected crashes or unhandled states, the following macro and panicking behaviors are denied in all target libraries:
* `clippy::unwrap_used = "deny"`
* `clippy::expect_used = "deny"`
* `clippy::panic = "deny"`

All potential failures must use explicit Rust error propagation (`Result<T, E>` + `?`).

### Complexity Restrictions
To maintain readability and clean separation of concerns, the compiler flags complex constructs:
* `clippy::too_many_lines = "warn"` (warns when a single function exceeds 100 lines).
* `clippy::cognitive_complexity = "warn"` (warns when control paths are nested or highly branching).

---

## 🧪 Developer Workflow & Local Loops

### 1. Build and Test Locally (on `ms02`)

The primary development environment is the `ms02` hypervisor. All compilation, check, and test commands must run directly on the `ms02` host via SSH.

> [!CAUTION]
> Avoid running `cargo build` or `cargo test` from macOS over the NFS mount. Doing so can hang or cause file system lock conflicts.

```bash
# 1. SSH into the development hypervisor
ssh ms02

# 2. Source the Rust toolchain environment
source ~/.cargo/env
cd ~/Workspace/microscaler/cylon-regenesis

# 3. Format, lint, compile, and run tests
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets --all-features
cargo test --workspace
```

### 2. Validate Actions Workflows Locally with `act`

The GitHub Actions validation pipeline can be run locally using `act` within docker containers on `ms02`.

> [!WARNING]
> macOS automatically generates hidden AppleDouble metadata files (`._*`) over NFS mounts. These metadata files corrupt `act`'s workflow configuration parsing. You must delete them prior to invoking `act`.

```bash
# Clean AppleDouble files and execute the serial validation workflow
find .github -name "._*" -delete && ~/bin/act -j pipeline
```

---

## 🚀 CI/CD Pipeline

All integration checks run in a **single consolidated workflow file** ([ci.yml](file:///Users/casibbald/Workspace/remote/microscaler/cylon-regenesis/.github/workflows/ci.yml)) with jobs executing sequentially in serial:

1. **Environment Verification:** Validates version constraints, checks shell script formatting via `shellcheck`, validates the `justfile`, checks `systemd` syntax, and validates local configs.
2. **Rust Verification:** Sets up a stable Rust environment, verifies compilation with `cargo check`, runs workspace lints via `clippy`, and executes all unit and integration tests.
3. **Release Phase:** Triggered automatically upon tag push (`v*`) or manual dispatch. It bundles the `regenesis-agent` scripts, configuration assets, and systemd service files into a tarball and publishes it to GitHub Releases.
