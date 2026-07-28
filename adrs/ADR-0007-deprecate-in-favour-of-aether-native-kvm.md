# ADR-0007: Deprecate Regenesis in favour of Aether native KVM

- **Status:** Accepted
- **Date:** 2026-07-28
- **Supersedes:** ADR-0001, ADR-0002, ADR-0004, ADR-0005, ADR-0006

## Context

Regenesis planned an independent Raft control plane and implemented a local
Multipass controller plus a shell host agent. The control plane was never
implemented, the controller was not operationally deployed, and its lifecycle
contracts duplicate Aether's placement, storage, fencing and recovery model.

The platform no longer wants Multipass as either a development path or an
approximation of production. Aether takes precedence.

One useful build pattern remains outside Regenesis: composing a guest
filesystem with a Dockerfile before turning it into a VM disk is clearer and
more reproducible than installing the full system through Cloud-Init.

## Decision

1. Deprecate and freeze `cylon-regenesis` pending archive.
2. Use Aether's direct QEMU/KVM implementation as the primary VM runtime.
3. Build guest filesystems with Dockerfiles in `cylon-images`, package them as
   versioned OCI KVM bundles, and let `aetherd` materialize them without a
   Docker daemon.
4. Use Cloud-Init only for small per-instance metadata and configuration.
5. Do not migrate the Multipass controller, `regenesis-agent`, Raft Hub plans,
   direct Hub registration/drain/watchdog, or duplicate iPXE artifact.
6. Keep `cylon-far` only as an optional subordinate compatibility adapter where
   Cylon-specific FAR mechanics are still required.

## Consequences

- Aether is the only authority for infrastructure admission and lifecycle.
- Development and production exercise the same native KVM path.
- `cylon-images` and `cylon-skills` remain compatible build inputs without
  becoming infrastructure authorities.
- Existing Regenesis code and plans are historical reference only.
- Active sibling dependencies must be removed before the repository is
  archived.
