# ADR-0004: Repo Boundaries

## Status

**Accepted** — 2026-06-13

## Context

CRP spans five repos. Without boundaries, hub logic, host daemon, images, and datacenter ops blur together.

## Decision

```
┌─────────────────────────────────────────────────────────────────┐
│                        cylon-regenesis                          │
│  regenesis-hub (Raft) │ regenesis-agent │ iPXE contracts │ docs │
└───────────────┬─────────────────────────────┬───────────────────┘
                │ gRPC mTLS                   │ BootIntent CRDs
                ▼                             ▼
┌───────────────────────────┐     ┌─────────────────────────────┐
│ cylon/crates/cylon      │     │ DCops                        │
│ Host daemon on node       │     │ iPXE, IPAM, Kea, NetBox      │
│ Firecracker, OCI, vsock   │     │ BootProfile / BootIntent     │
└───────────────┬───────────┘     └─────────────────────────────┘
                │ pulls guest OCI
                ▼
┌───────────────────────────┐     ┌─────────────────────────────┐
│ cylon-images/container/   │     │ cylon (remainder)          │
│ GHCR kernel + rootfs      │     │ engine, portal, platform     │
└───────────────────────────┘     └─────────────────────────────┘
```

| Artifact | Canonical repo | Notes |
|---|---|---|
| Hub Raft state, `/v2/agents`, scheduling | **cylon-regenesis** (migrate from `cylon/crates/resurrection-hub`) | OpenRaft + axum |
| `CylonService` host implementation | **cylon/crates/cylon** | Stays — heavy Firecracker integration |
| `cylon.proto` | **cylon** (source of truth) | regenesis-hub depends on published crate or path dep |
| Guest `vmlinux` + rootfs OCI | **cylon-images** | GHCR |
| Resurrection-node **host OS** netboot image | **cylon-regenesis** (build) + DCops (serve) | Not guest rootfs |
| Multipass dev cloud-init | **cylon-images/multipass** until Phase 1 complete | Parity spec in regenesis docs |
| Platform daemon, Postgres, portal | **cylon** | Kind only |
| Operator Tilt/Kind | **cylon-local-infra** | |

## Migration rule

Extract **resurrection-hub** to `cylon-regenesis/crates/regenesis-hub` in Phase 3. Cylon retains a thin client or git submodule pointer until cutover. No duplicate hub implementations.

## Consequences

- Cross-repo version matrix required ([ARCHITECTURE](../docs/ARCHITECTURE.md) § Version matrix).
- CI in each repo validates contract compatibility.

## References

- [REPO-MAP](../docs/REPO-MAP.md)
