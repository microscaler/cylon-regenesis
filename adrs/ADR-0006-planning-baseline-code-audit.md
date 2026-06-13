# ADR-0006: Planning baseline — code audit snapshot

## Status

**Accepted** — 2026-06-13

## Context

The master plan ([docs/plan/MASTER-PLAN.md](../docs/plan/MASTER-PLAN.md)) must reflect **actual** cylon implementation, not aspirational RESURRECTION-HUB-PRD checkboxes alone.

## Decision

Treat [02-gap-analysis-current-state.md](../docs/plan/02-gap-analysis-current-state.md) as the authoritative **implementation snapshot** for planning. Key findings:

1. **Substantial hub logic already exists** in `cylon/crates/resurrection-hub` (Raft, batch allocator, rejoin, drain, OTEL).
2. **Cylon host detached watchdog** is implemented in `crates/cylon/src/main.rs`.
3. **Greenfield work** is primarily: regenesis-agent, iPXE/DCops, hub crate migration, hardening gaps (S3 GC, GHCR auth, rejoin DeleteCylonVm).
4. RESURRECTION-HUB-PRD items marked `[x]` may exceed code — verify against gap doc before claiming done.

## Consequences

- Phase priorities: agent first, migration second, PXE parallel on DCops dependency.
- WBS task IDs in plan/13 are canonical for issue tracking.

## Review

Update this ADR when major epics complete (Phase 1, 3, 2 sign-off).
