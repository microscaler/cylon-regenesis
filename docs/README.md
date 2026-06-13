# Documentation index

## Start here

| Document | Purpose |
|---|---|
| **[plan/MASTER-PLAN.md](plan/MASTER-PLAN.md)** | **Excruciating-detail implementation plan (15 docs)** |
| [ARCHITECTURE.md](ARCHITECTURE.md) | System design, data flows, component diagram |
| [PRD.md](PRD.md) | Product requirements, acceptance criteria, phase index |
| [REPO-MAP.md](REPO-MAP.md) | Sibling repos, ownership, dependency arrows |

## Requirements baseline

| Document | Purpose |
|---|---|
| [proposals/README.md](proposals/README.md) | Flintlock distributed architecture lineage |
| [proposals/flintlock-requirements-mapping.md](proposals/flintlock-requirements-mapping.md) | 15 proposal docs → cylon-regenesis components |

## Host regenesis (iPXE + first boot)

| Document | Purpose |
|---|---|
| [host-regenesis/README.md](host-regenesis/README.md) | Overview |
| [host-regenesis/ipxe-provisioning.md](host-regenesis/ipxe-provisioning.md) | iPXE chain, scripts, artifacts |
| [host-regenesis/dcops-integration.md](host-regenesis/dcops-integration.md) | BootIntent, BootProfile, IPClaim |
| [host-regenesis/first-boot-sequence.md](host-regenesis/first-boot-sequence.md) | regenesis-agent steps |
| [host-regenesis/cloud-init-parity.md](host-regenesis/cloud-init-parity.md) | Multipass equivalence checklist |

## Control plane (regenesis hub)

| Document | Purpose |
|---|---|
| [control-plane/README.md](control-plane/README.md) | Hub overview |
| [control-plane/raft-consensus.md](control-plane/raft-consensus.md) | OpenRaft, log, snapshots |
| [control-plane/scheduling-and-bidding.md](control-plane/scheduling-and-bidding.md) | Allocator, batch bidding |
| [control-plane/api-v2-and-proxy.md](control-plane/api-v2-and-proxy.md) | HTTP API, gRPC proxy |
| [control-plane/fault-tolerance.md](control-plane/fault-tolerance.md) | Partitions, detach, rejoin, resurrection |
| [control-plane/observability.md](control-plane/observability.md) | OTEL, metrics, tracing |
| [control-plane/storage-and-gc.md](control-plane/storage-and-gc.md) | S3 snapshots, GC policy |

## Implementation phases

| Document | Purpose |
|---|---|
| [phases/README.md](phases/README.md) | Roadmap overview |
| [phases/phase-0-docs-and-contracts.md](phases/phase-0-docs-and-contracts.md) | **Current** — docs, ADRs, contracts |
| [phases/phase-1-multipass-parity.md](phases/phase-1-multipass-parity.md) | regenesis-agent + Multipass |
| [phases/phase-2-dcops-ipxe-dev.md](phases/phase-2-dcops-ipxe-dev.md) | Lab bare metal + DCops |
| [phases/phase-3-control-plane-extraction.md](phases/phase-3-control-plane-extraction.md) | Move resurrection-hub |
| [phases/phase-4-fault-tolerance.md](phases/phase-4-fault-tolerance.md) | Production resilience |
| [phases/phase-5-production-bare-metal.md](phases/phase-5-production-bare-metal.md) | DC rollout |

## ADRs

See [`../adrs/`](../adrs/).
