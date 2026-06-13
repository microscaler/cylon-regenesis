# Fault tolerance

**Flintlock proposals 04, 05, 07, 09** — detailed in [RESURRECTION-HUB-PRD](../../tiffany/docs/RESURRECTION-HUB-PRD.md) Phase 2.

## 1. Node heartbeat and offline (04, 07)

| Parameter | Value |
|---|---|
| Heartbeat interval | 5s |
| Miss threshold | 3 |
| Action | `MarkNodeOffline`, zero capacity |

Leader reschedules agents:

1. Hibernate or assume snapshot exists
2. Re-bid on healthy nodes
3. `ResurrectCylonVm` or `CreateCylonVm` from S3

**Acceptance:** network kill → reschedule <15s.

## 2. Detached host GC (05)

Implemented on **cylon host** (tiffany), orchestrated by hub policy.

| Parameter | Value |
|---|---|
| Hub ping interval | 10s |
| Detach threshold | 30s no leader reachability |
| Detached action | HibernateAll or SIGKILL policy |

Host disables vsock egress proxy in detached mode.

## 3. Network partition (07)

| Scenario | Behavior |
|---|---|
| Hub minority partition | Stops accepting writes (Raft) |
| Node isolated from hub | Detached GC |
| Node isolated but thinks hub OK | Prevented by hub-side offline mark |

No split-brain agent writes to external SaaS.

## 4. Rejoin reconciliation (09)

```
POST /v2/nodes/rejoin
Request:  { node_id, local_vm_ids: [...] }
Response: { terminate: [...], resume: [...], register_ok: true }
```

Host executes terminate list before accepting new bids.

## 5. VM resurrection (04)

Not cold reinstall — **Firecracker snapshot restore**:

1. Hub reads `snapshot_uri` from Raft
2. Target node `ResurrectCylonVm`
3. Raft updates `agent.node_id`

## Chaos test matrix

| Test | Pass criteria |
|---|---|
| Kill node-1 network | Agents moved <15s |
| Kill hub leader | Raft elects <5s |
| Partition node-2 from hub | VMs stopped <30s |
| Rejoin node-2 | Duplicates terminated |

## References

- [first-boot-sequence.md](../host-regenesis/first-boot-sequence.md) — registration
- `tiffany/crates/cylon/src/lifecycle.rs`
