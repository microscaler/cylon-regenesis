# Phase 4 — Fault tolerance

**Depends on:** Phase 3 (hub in regenesis repo)  
**Goal:** Implement and **chaos-test** RESURRECTION-HUB-PRD Phase 2–4 features.

## Scope (from RESURRECTION-HUB-PRD)

| Item | Doc |
|---|---|
| Heartbeat + offline + reschedule | [fault-tolerance.md](../control-plane/fault-tolerance.md) |
| Detached host watchdog | tiffany `crates/cylon` |
| Rejoin reconciliation | `/v2/nodes/rejoin` |
| Batch bidding | [scheduling-and-bidding.md](../control-plane/scheduling-and-bidding.md) |
| Raft snapshots | [raft-consensus.md](../control-plane/raft-consensus.md) |
| CryoSleep GC | [storage-and-gc.md](../control-plane/storage-and-gc.md) |
| OTEL tracing | [observability.md](../control-plane/observability.md) |

## Test harness

Automated chaos suite (Rust integration or pytest via farm):

| Scenario | Method |
|---|---|
| Node network drop | `iptables DROP` on Multipass |
| Hub leader kill | `kubectl delete pod` |
| Split hub | Network partition simulation |
| Rejoin | Restore network + call rejoin |

## Acceptance criteria

- [ ] All chaos matrix rows pass ([fault-tolerance.md](../control-plane/fault-tolerance.md))
- [ ] Distributed trace visible Hub → host → FC
- [ ] 7-day soak: Raft log bounded

## Next phase

[Phase 5 — Production bare metal](phase-5-production-bare-metal.md)
