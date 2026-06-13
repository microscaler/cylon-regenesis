# Raft consensus

**Flintlock proposal 01, 10** — cluster-wide consistent state.

## Implementation

| Item | Choice |
|---|---|
| Library | **OpenRaft** (existing in resurrection-hub) |
| State | In-memory `HashMap` + periodic snapshot (Phase 4 hardening) |
| Transport | gRPC between hub peers (mTLS) |
| Client writes | `client_write` for all mutating API ops |

## State machine entities

```rust
// Conceptual — matches tiffany resurrection-hub evolution

CylonNode {
  id, address, status: Online|Offline|Draining,
  capacity, available,
  last_heartbeat,
  mtls_cert_fingerprint,
}

Agent {
  id, state: Running|CryoSleep|Failed,
  node_id,           // authoritative placement
  rootfs_ref,        // GHCR OCI
  snapshot_uri,      // S3 when hibernated
  created_at, updated_at,
}
```

## Log commands

| Command | Trigger |
|---|---|
| `RegisterNode` | `POST /v2/register` |
| `UpdateNodeHeartbeat` | Node ping |
| `MarkNodeOffline` | Missed heartbeats |
| `CreateAgent` | Scheduler decision |
| `MoveAgent` | Failover reschedule |
| `HibernateAgent` | Detached GC / user |
| `RejoinNode` | `POST /v2/nodes/rejoin` |
| `DrainNode` | Admin op |

## Leader duties

Only leader:

- Runs scheduler batch loop
- Sends gRPC to cylon hosts for placement
- Runs S3 GC sweep
- Accepts node registrations

Followers:

- Forward writes to leader
- Serve read-only registry (optional)

## Snapshotting (proposal 10)

| Parameter | Initial value |
|---|---|
| Snapshot threshold | every 10_000 log entries |
| Snapshot storage | PVC or S3 object |
| Recovery | Install snapshot + replay tail |

Acceptance: hub memory/disk flat over 7-day soak ([RESURRECTION-HUB-PRD](../../tiffany/docs/RESURRECTION-HUB-PRD.md) 3.2).

## Testing

- Kill leader pod → new leader <5s
- Split hub quorum → minority partition rejects writes
- Snapshot restore → registry intact

## References

- `tiffany/crates/resurrection-hub/`
- OpenRaft docs
