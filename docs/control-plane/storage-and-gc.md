# Storage and garbage collection

**Flintlock proposals 12, 14**

## Agent snapshots (CryoSleep)

| Item | Detail |
|---|---|
| Backend | S3-compatible via `object_store` crate |
| Content | Firecracker memory + disk snapshot |
| URI | Stored in Raft `Agent.snapshot_uri` |
| Trigger | `HibernateCylonVm`, detached GC, admin |

## Global GC policy (proposal 12)

Leader-only background task:

| Rule | Default |
|---|---|
| CryoSleep TTL | 7 days without API/UI touch |
| Action | Deregister agent + delete S3 objects |
| Override | Admin `POST /v2/agents/{id}/retain` |

Acceptance: dormant agents purged; S3 usage drops ([RESURRECTION-HUB-PRD](../../tiffany/docs/RESURRECTION-HUB-PRD.md) 4.1).

## Graceful migration (proposal 14)

Future — Phase 4+:

1. Hub marks node `Draining`
2. For each agent: pause FC → stream memory to target host → resume
3. Update Raft placement

Requires Firecracker snapshot/live migration research — not Phase 1–3.

## Host regenesis storage

Host reprovision **wipes local** `/var/lib/cylon/` VM state — agents must be hibernated or rescheduled before triggering BootIntent reinstall.

## References

- `tiffany/crates/resurrection-hub/Cargo.toml` (object_store)
- Flintlock doc 14
