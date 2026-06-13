# Control plane — regenesis hub

Central **Raft-backed** orchestrator for CRP. Migrates from `tiffany/crates/resurrection-hub`.

## Responsibilities

- Global agent registry and lifecycle
- Resurrection node capacity tracking
- Scheduling (bidding)
- HTTP API v2 for platform + proxy to hosts
- S3 snapshot metadata
- Fault tolerance orchestration

## Does not

- Run Firecracker (delegates to cylon host)
- Build guest OCI images
- Serve iPXE (DCops)

## Subdocuments

| Doc | Flintlock # |
|---|---|
| [raft-consensus.md](raft-consensus.md) | 01, 10 |
| [scheduling-and-bidding.md](scheduling-and-bidding.md) | 02, 08 |
| [api-v2-and-proxy.md](api-v2-and-proxy.md) | 03 |
| [fault-tolerance.md](fault-tolerance.md) | 04, 05, 07, 09 |
| [observability.md](observability.md) | 13 |
| [storage-and-gc.md](storage-and-gc.md) | 12, 14 |

## Deployment

| Phase | Form |
|---|---|
| Now | Kind pod `resurrection-hub:14000` (tiffany Tilt) |
| Phase 3+ | Container from cylon-regenesis CI |
| Phase 5 | 3-node Raft cluster on mgmt hardware |

## Code migration

See [phase-3-control-plane-extraction.md](../phases/phase-3-control-plane-extraction.md).
