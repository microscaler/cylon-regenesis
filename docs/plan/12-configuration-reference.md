# 12 — Configuration reference

## regenesis-hub

| Variable | Default | Description |
|---|---|---|
| `HUB_NODE_ID` | `1` | Raft peer id |
| `CYLON_CERTS_DIR` | `dev/certs` | mTLS client certs to nodes |
| `CYLON_GRPC_ENDPOINT` | unset | Optional ping target |
| `RUST_LOG` | `info` | tracing filter |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | unset | tracing export |

Listen: `:14000` (hardcoded main.rs today)

## Cylon host

| Variable | Default | Description |
|---|---|---|
| `CYLON_NODE_ID` | `cylon-node-1` | |
| `CYLON_GRPC_ENDPOINT` | `http://127.0.0.1:50051` | hub dials this |
| `HUB_API_ENDPOINT` | `http://127.0.0.1:14000` | watchdog |
| `CYLON_AVAILABLE_MEMORY_MB` | `8192` | register payload |
| `CYLON_AVAILABLE_VCPU` | `8` | |
| `CYLON_CERTS_DIR` | — | gRPC mTLS |
| `GITHUB_TOKEN` | — | OCI auth target |

Listen gRPC: `:50052` typical (Multipass bridge)

## regenesis-agent

See [06-regenesis-agent-spec.md](06-regenesis-agent-spec.md) — `/etc/regenesis/config.yaml`

## Multipass / cylon just

| Recipe | Purpose |
|---|---|
| `resurrection-nodes-deploy-host-daemon` | env + certs (Phase 1 until agent full) |
| `resurrection-nodes-status` | health check |
| `resurrection-nodes-smoke-cylon-rootfs-ubuntu` | GHCR E2E |

## File paths (resurrection node)

| Path | Mode | Content |
|---|---|---|
| `/etc/cylon/github-token` | 0600 | PAT |
| `/etc/cylon/release-pin` | 0644 | `latest` or tag |
| `/etc/cylon/host.env` | 0644 | cylon host env |
| `/etc/cylon/certs/*` | 0600/644 | mTLS |
| `/etc/regenesis/config.yaml` | 0644 | agent config |
| `/var/lib/regenesis/configured` | 0644 | marker timestamp |
| `/home/cylon/cylon-images/vmlinux` | 644 | guest kernel |

## GHCR pins (2026-06)

| Image | Tag |
|---|---|
| cylon-kernel | `6.1.102` |
| cylon-rootfs-ubuntu | `latest` |

## DCops

| CRD field | Example |
|---|---|
| BootProfile.spec.kernel | HTTP URL |
| BootIntent.spec.macAddress | lowercase hex with colons |

## Kind / Tilt

| Service | Port forward |
|---|---|
| resurrection-hub | 14000 |
| portal | 5173 |

See `cylon/Tiltfile`, `cylon-local-infra`.
