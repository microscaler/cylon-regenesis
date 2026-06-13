# 01 — Vision & principles

## Vision

**Cylon Regenesis** is the Microscaler control plane that:

1. **Schedules** AI agent Firecracker microVMs across a fleet of resurrection nodes.
2. **Resurrects** agents from CryoSleep snapshots after failure or idle periods.
3. **Regenerates** bare-metal hosts via standard iPXE when hardware is new, RMA'd, or corrupted.

It replaces Liquidmetal + Flintlock for CRP without upstream governance constraints.

## Design tenets

### T1 — Agents are not Kubernetes nodes

MicroVMs run `cylon-runtime` + `mayfly-guest`. They never join a customer K8s cluster. Scheduling optimizes **agent density**, not pod CNI.

### T2 — Two provisioning planes

| Plane | Mechanism | Delivers |
|---|---|---|
| **Host** | iPXE + DCops + regenesis-agent | Cylon host daemon on bare metal |
| **Guest** | Hub `CreateCylonVm` + GHCR OCI | Agent rootfs inside Firecracker |

Never boot guest rootfs via PXE.

### T3 — GitOps for metal, API for agents

- **DCops Git** owns MAC → boot profile, IP claims, boot lifecycle lock.
- **regenesis-hub API** owns agent create/hibernate/resurrect/migrate.

### T4 — Fail closed on partition

- Hub minority partition: no writes (Raft quorum).
- Node isolated from hub: **Detached Mode** — pause/kill microVMs, no egress.
- Rejoin: manifest reconciliation before accepting bids.

### T5 — Single trace, many hops

Every agent operation must trace: Portal → Hub → Raft → gRPC → Firecracker UDS → vsock guest.

### T6 — Minimize scope per repo

See [ADR-0004](../../adrs/ADR-0004-repo-boundaries.md). regenesis-hub migrates here; cylon host stays in cylon until a future split is justified.

## Non-goals (repeated for planners)

- Flintlock API compatibility
- Tinkerbell Workflow / Hook
- CAPMVM / Cluster API for agents
- Multi-tenant SaaS isolation (Phase 1–5)
- In-guest compilation (cargo on node)

## Success definition (5-year horizon)

| Capability | Measure |
|---|---|
| Host replace | Rack slot empty → registered node <15 min |
| Agent failover | Node death → agent running elsewhere <15s |
| Idle cost | CryoSleep → S3, zero RAM on node |
| Ops toil | Zero manual Multipass for production |
| Audit | Git history for every boot profile change |

## Terminology

| Term | Meaning |
|---|---|
| **regenesis-hub** | Raft control plane (rename on migration from resurrection-hub) |
| **regenesis-agent** | First-boot host provisioner |
| **resurrection node** | Bare metal (or Multipass dev) running cylon host |
| **agent** | Logical AI workload = one Firecracker microVM |
| **CryoSleep** | Hibernated agent; memory on S3 |
| **Detached Mode** | Host lost hub; VMs frozen |
| **BootIntent** | DCops CRD: MAC → boot profile + lifecycle |
