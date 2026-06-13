# ADR-0002: Flintlock Replacement — Not a Fork

## Status

**Accepted** — 2026-06-13

## Context

Microscaler submitted a [Flintlock distributed architecture enhancement proposal](../docs/proposals/README.md) to Liquid Metal (June 2025), choosing collaboration over forking. Operational reality: upstream velocity and CRP-specific semantics (agent resurrection, cylon-skills guest, CryoSleep snapshots) require **full control**.

Tiffany [CRP PRD](../../tiffany/docs/CYLON-RESURRECTION-PLATFORM-PRD.md) §0 already maps Flintlock responsibilities to `crates/cylon` + `resurrection-hub`. **cylon-regenesis** consolidates the control plane and host regenesis into a dedicated repo.

## Decision

Build a **greenfield replacement** that implements the proposal's *capabilities*, not Flintlock's API or codebase.

| Flintlock concept | CRP replacement |
|---|---|
| Per-host isolated state | OpenRaft cluster state in **regenesis hub** |
| Flintlock gRPC | **`CylonService`** gRPC (`cylon.proto`) to host |
| Global scheduling | Hub allocator + bidding ([doc 02](../docs/control-plane/scheduling-and-bidding.md)) |
| Unified API | Hub **`/v2/*`** + proxy to authoritative host |
| VM persistence | CryoSleep snapshots via `object_store` (S3) |
| Host reprovisioning | **iPXE + regenesis-agent** ([doc 06](../docs/host-regenesis/ipxe-provisioning.md)) |
| Quickstart kernel blob | **`ghcr.io/microscaler/cylon-kernel:6.1.102`** (guest) |

We do **not** maintain API adapters for Flintlock clients.

## Consequences

- Faster iteration on CRP semantics.
- No upstream governance dependency for critical path.
- Flintlock proposal docs remain the **requirements baseline** in `docs/proposals/`.

## References

- [Flintlock requirements mapping](../docs/proposals/flintlock-requirements-mapping.md)
- [ARCHITECTURE](../docs/ARCHITECTURE.md)
