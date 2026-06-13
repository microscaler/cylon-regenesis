# Cloud-init parity checklist

Multipass dev path: [`cylon-images/multipass/cloud-init.yaml`](../../../cylon-images/multipass/cloud-init.yaml)

**regenesis-agent** must produce equivalent host state. Check each item before Phase 1 sign-off.

## Users and permissions

| # | cloud-init | regenesis-agent | ✓ |
|---|---|---|---|
| 1 | User `cylon` in groups `sudo`, `kvm` | same | |
| 2 | NOPASSWD sudo | same | |
| 3 | `setfacl -m u:cylon:rw /dev/kvm` | same | |
| 4 | `/home/cylon` owned by cylon | same | |

## Packages

| # | Package | ✓ |
|---|---|---|
| 5 | qemu-kvm | |
| 6 | cpu-checker | |
| 7 | curl, jq, ca-certificates, libssl3 | |
| 8 | acl, iproute2 | |
| 9 | crane (GHCR) — added in regenesis vs older cloud-init | |

## Firecracker

| # | Item | ✓ |
|---|---|---|
| 10 | firecracker v1.10.1 at `/usr/bin/firecracker` | |
| 11 | jailer v1.10.1 at `/usr/bin/jailer` | |

## Guest kernel

| # | Item | ✓ |
|---|---|---|
| 12 | `/home/cylon/cylon-images/vmlinux` from GHCR `cylon-kernel:6.1.102` | |
| 13 | Not S3 quickstart blob | |
| 14 | Mode 644, owner cylon | |

## Cylon host binary

| # | Item | ✓ |
|---|---|---|
| 15 | `/etc/cylon/github-token` present before install | |
| 16 | `/etc/cylon/release-pin` (default latest) | |
| 17 | `/usr/local/bin/cylon` from GitHub Releases | |
| 18 | sha256 verified | |

## systemd

| # | Item | ✓ |
|---|---|---|
| 19 | `/etc/systemd/system/cylon-host.service` | |
| 20 | `EnvironmentFile=-/etc/cylon/host.env` | |
| 21 | enabled + started | |
| 22 | health `:8080/health` OK | |

## Hub integration (regenesis extension)

| # | Item | cloud-init today | regenesis-agent |
|---|---|---|---|
| 23 | Hub registration | manual / separate deploy | **POST /v2/register** |
| 24 | mTLS certs | `resurrection-nodes-deploy-host-daemon` | bundled or register response |
| 25 | Node env per host | `host.env.node-*` | written at register |

Items 23–25 are **beyond** current cloud-init — regenesis-agent adds them so Multipass nodes self-register.

## Verification command (ms02)

```bash
cd ~/Workspace/microscaler/tiffany
just resurrection-nodes-status
just resurrection-nodes-smoke-grpc
```

## Sign-off

| Role | Date | Notes |
|---|---|---|
| Platform | | |
| DCops | | iPXE not required for Phase 1 |
