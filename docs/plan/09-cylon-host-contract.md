# 09 — Cylon host contract

**Owner repo:** `cylon/crates/cylon`  
**regenesis-hub depends on this contract — do not fork proto here long-term.**

## Responsibilities split

| Concern | regenesis-hub | cylon host |
|---|---|---|
| Placement decision | ✅ | |
| OCI pull + ext4 | | ✅ |
| Firecracker API | | ✅ |
| vsock egress proxy | | ✅ |
| Detached watchdog | | ✅ |
| Hub heartbeat | | ✅ |
| First-boot OS install | regenesis-agent | |

## Environment contract

| Variable | Required | Description |
|---|---|---|
| `CYLON_NODE_ID` | yes | Matches hub registry |
| `CYLON_GRPC_ENDPOINT` | yes | URL hub uses to dial |
| `HUB_API_ENDPOINT` | yes | Watchdog target |
| `CYLON_AVAILABLE_MEMORY_MB` | yes* | *auto future |
| `CYLON_AVAILABLE_VCPU` | yes* | |
| `CYLON_CERTS_DIR` | prod | mTLS material |
| `GITHUB_TOKEN` | for GHCR | OCI auth future |

## gRPC service level

| RPC | Latency target | Idempotent |
|---|---|---|
| CreateCylonVm | <120s cold OCI | no — duplicate id error |
| GetCylonVm | <100ms | yes |
| DeleteCylonVm | <10s | yes |
| HibernateCylonVm | <30s | yes |
| ResurrectCylonVm | <60s | yes |
| MigrateCylonVm | minutes | no |

## OCI requirements

- Support `ghcr.io/microscaler/cylon-rootfs-ubuntu:latest`
- Support dev registry `*:5001` with HTTP
- **Gap:** RegistryAuth for private GHCR — must read token from env/file

## Detached mode contract

When hub unreachable 30s:

1. Pause all running FC instances
2. Stop accepting new gRPC Create (return Unavailable)
3. Continue retry register every 5s
4. On recovery: rejoin + execute kill list properly (**DeleteCylonVm**)

## Migration contract

`MigrateCylonVm` must:

1. Pause source VM
2. Stream memory chunks to target `ReceiveMigration`
3. Target resumes VM
4. Source deletes VM

**Status:** stub — epic in cylon, tracked REG-HOST-4.x.

## Health endpoints

| Path | Purpose |
|---|---|
| GET `:8080/health` | regenesis-agent verify |
| GET `:8080/metrics` | **target** Prometheus |

## Files on disk (conventions)

| Path | Purpose |
|---|---|
| `/home/cylon/cylon-images/vmlinux` | Guest kernel |
| `/var/lib/cylon/{vm_id}_rootfs.ext4` | Guest disk |
| `/tmp/firecracker-{vm_id}.sock` | FC API socket |
| `/tmp/cylon/` | worktrees |

## Version compatibility matrix

| cylon host | regenesis-hub | Notes |
|---|---|---|
| v0.1.0 | 0.1.x | ms02 today |
| v0.1.1+ | 0.1.x | boot/OCI fixes |
| v0.2.0 | 0.2.x | typed Raft + GHCR auth |

Breaking proto changes require major bump both sides.
