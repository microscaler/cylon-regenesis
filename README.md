# Cylon Regenesis

**Distributed Firecracker orchestration and host regeneration for the Cylon Resurrection Platform (CRP).**

Microscaler-owned replacement for [Liquidmetal + Flintlock](https://github.com/liquidmetal-dev/flintlock) — built for agent microVMs, resurrection snapshots, and our operational model. Not a fork; not CAPMVM-compatible.

## What this repo is

| Layer | Responsibility | Status |
|---|---|---|
| **Host regenesis** | iPXE provisioning, first-boot, secure Hub rejoin | Planned — [docs/host-regenesis/](docs/host-regenesis/) |
| **Control plane** | Raft hub, scheduling, API v2, fault tolerance | Migrate from `tiffany/crates/resurrection-hub` |
| **Contracts** | Hub ↔ host ↔ DCops integration specs | This repo |

## What lives elsewhere

| Repo | Role |
|---|---|
| [`tiffany`](../tiffany/) | Cylon **host daemon** (`crates/cylon`), agent runtime, portal |
| [`cylon-images`](../cylon-images/) | Firecracker **guest** kernel + OCI rootfs (GHCR) |
| [`DCops`](../DCops/) | Datacenter GitOps — **iPXE, DHCP, IPAM** (not Tinkerbell) |
| [`cylon-local-infra`](../cylon-local-infra/) | Kind, Tilt, operator runbooks |

## Quick links

- **[Master plan (excruciating detail)](docs/plan/MASTER-PLAN.md)**
- [Architecture](docs/ARCHITECTURE.md)
- [PRD & phases](docs/PRD.md)
- [ADRs](adrs/README.md)
- [Flintlock proposal mapping](docs/proposals/flintlock-requirements-mapping.md)
- [iPXE + DCops integration](docs/host-regenesis/dcops-integration.md)

## Lineage

Requirements baseline: Microscaler [Flintlock Distributed Architecture proposal](docs/proposals/README.md) (June 2025). We implement the same *capabilities* with CRP-specific shape: `cylon.proto`, OpenRaft hub, GHCR artifacts, DCops iPXE.
