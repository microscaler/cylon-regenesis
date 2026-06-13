# ADR-0001: Scope & Non-Goals

## Status

**Accepted** — 2026-06-13

## Context

Cylon Regenesis is the Microscaler-owned distributed microVM control plane and host lifecycle system. Without explicit boundaries it will overlap `cylon`, `cylon-images`, and `DCops`.

## Decision

### In scope

1. **Host regenesis** — provision and reprovision **resurrection nodes** (Cylon host OS + daemon) via iPXE/DCops.
2. **Control plane** — Raft-backed hub: agent registry, scheduling, API v2, fault tolerance (migrated from `cylon/crates/resurrection-hub`).
3. **Integration contracts** — Hub ↔ Cylon host ↔ DCops BootIntent/BootProfile ↔ storage (S3/object_store).
4. **Operational docs** — runbooks, phase plans, Flintlock-requirements traceability.

### Explicit non-goals

| Non-goal | Rationale |
|---|---|
| Flintlock API compatibility | We use `cylon.proto` / HTTP `/v2` — see ADR-0002 |
| Tinkerbell / Hegel / Hook | DCops + standard iPXE — see ADR-0003 |
| CAPMVM / agents as K8s nodes | CRP model — agents are Firecracker guests |
| Guest rootfs/kernel CI | Owned by `cylon-images` |
| Agent LLM runtime | Owned by `cylon` (`engine`, `cylon-skills`) |
| Talos cluster lifecycle | Owned by `DCops` Phase 2+ (CAPI) |
| Generic multi-tenant cloud | Single-tenant CRP first |

## Consequences

- Clear ownership reduces duplicate work across repos.
- Host regenesis must integrate DCops CRDs, not invent parallel IPAM/PXE.

## References

- [PRD](../docs/PRD.md)
- [REPO-MAP](../docs/REPO-MAP.md)
- DCops [ADR-001](../../DCops/ADRs/ADR-001-Scope_and_Non-Goals.md)
