# Flintlock proposal → cylon-regenesis mapping

Status key: **Done** (in cylon today) | **Planned** (this repo) | **DCops** | **N/A**

| # | Flintlock proposal | CRP component | Status | Doc |
|---|---|---|---|---|
| 01 | Raft consensus | regenesis-hub OpenRaft | Done → migrate | [raft-consensus](../control-plane/raft-consensus.md) |
| 02 | Distributed scheduling / bidding | Hub allocator | Done → migrate | [scheduling](../control-plane/scheduling-and-bidding.md) |
| 03 | Unified API + proxy | Hub `/v2` + gRPC proxy | Done → migrate | [api-v2](../control-plane/api-v2-and-proxy.md) |
| 04 | Host failure handling | Hub offline + agent reschedule | Planned | [fault-tolerance](../control-plane/fault-tolerance.md) |
| 05 | Detached host GC | Cylon host watchdog | Planned (cylon) | [fault-tolerance](../control-plane/fault-tolerance.md) |
| 06 | Host regenesis / PXE | regenesis-agent + DCops iPXE | **Planned** | [ipxe-provisioning](../host-regenesis/ipxe-provisioning.md) |
| 07 | Network partition / split-brain | Hub quorum + node fence | Planned | [fault-tolerance](../control-plane/fault-tolerance.md) |
| 08 | Leader scheduling bottleneck | Batch bid queue | Planned | [scheduling](../control-plane/scheduling-and-bidding.md) |
| 09 | Host rejoin reconciliation | `POST /v2/nodes/rejoin` | Planned | [fault-tolerance](../control-plane/fault-tolerance.md) |
| 10 | Raft log snapshotting | OpenRaft snapshot tuning | Planned | [raft-consensus](../control-plane/raft-consensus.md) |
| 11 | Security / authorization | mTLS + RBAC | Partial | [ARCHITECTURE](../ARCHITECTURE.md) §7 |
| 12 | GC policy | CryoSleep TTL sweep | Planned | [storage-and-gc](../control-plane/storage-and-gc.md) |
| 13 | Observability | OTEL cross-boundary | Planned | [observability](../control-plane/observability.md) |
| 14 | Graceful VM migration | Firecracker memory pipe | Future | [storage-and-gc](../control-plane/storage-and-gc.md) |
| 15 | Config / operational clarity | Centralized hub config + ADRs | **In progress** | [phases](../phases/README.md) |

## CRP-specific additions (not in Flintlock)

| Capability | Where |
|---|---|
| Agent skills in guest rootfs | cylon-images |
| CryoSleep / S3 resurrection semantics | regenesis-hub + cylon host |
| VSock egress proxy with billing | cylon host |
| mayfly-guest orchestration | cylon + mayfly |
| GHCR guest artifacts | cylon-images |

## Cylon RESURRECTION-HUB-PRD crosswalk

| Hub PRD phase | Flintlock docs |
|---|---|
| Phase 1 Telemetry | 13 |
| Phase 2 Fault tolerance | 04, 05, 07, 09 |
| Phase 3 Scheduling | 08, 10 |
| Phase 4 Lifecycle | 12, 14 |

Host regenesis (Flintlock **06**) is **not** in RESURRECTION-HUB-PRD — owned entirely by cylon-regenesis.
