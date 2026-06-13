# First-boot sequence — regenesis-agent

**regenesis-agent** is the idempotent first-boot program that converts a fresh **host OS** into a **registered resurrection node**.

Runs as:

- Phase 1: `systemd` oneshot on Multipass (replaces cloud-init `runcmd` subset)
- Phase 2+: same binary invoked from autoinstall `late-commands`

## Inputs (environment / files)

| Input | Source | Required |
|---|---|---|
| `REGENESIS_HUB_URL` | autoinstall / cloud-init | yes |
| `REGENESIS_NODE_ID` | BootIntent name or generated | yes |
| `GITHUB_TOKEN` | `/etc/cylon/github-token` (0600) | yes (private releases) |
| `CYLON_RELEASE_PIN` | `/etc/cylon/release-pin` | default `latest` |
| `GHCR_KERNEL` | default `ghcr.io/microscaler/cylon-kernel:6.1.102` | yes |
| mTLS cert paths | hub registration response or pre-seeded | Phase 2 |

## Steps (ordered)

### 1. Preflight

```
- root or passwordless sudo
- /dev/kvm exists
- cpu-checker kvm-ok (or equivalent)
- network default route
```

Exit 0 if **already configured** marker present: `/var/lib/regenesis/configured`.

### 2. OS packages

Equivalent to cloud-init:

```bash
apt-get update
apt-get install -y qemu-kvm cpu-checker acl curl jq iproute2 ca-certificates libssl3 crane
```

`crane` used for GHCR guest kernel extract (same as `install-kernel-from-ghcr` in cylon-images).

### 3. User and KVM ACL

```
user: cylon (sudo, kvm)
setfacl -m u:cylon:rw /dev/kvm
```

### 4. Firecracker + jailer

Pin **v1.10.1** (match current cloud-init):

```
/usr/bin/firecracker
/usr/bin/jailer
```

Verify with `firecracker --version`.

### 5. Guest kernel (GHCR)

```
mkdir -p /home/cylon/cylon-images
crane auth login ghcr.io (if token present)
crane export ghcr.io/microscaler/cylon-kernel:6.1.102 | tar -xOf - vmlinux > .../vmlinux
chown cylon:cylon /home/cylon/cylon-images/vmlinux
```

**Not** the iPXE boot kernel — this is the Firecracker guest kernel.

### 6. Cylon host binary

Same logic as `install-cylon-from-release` in `cylon-images/multipass/cloud-init.yaml`:

- Download `cylon-linux-x86_64` + sha256 from Tiffany GitHub Releases (API asset URLs)
- Install `/usr/local/bin/cylon`

### 7. systemd unit

Install `cylon-host.service` + `/etc/cylon/host.env` from templates or hub-provided bundle.

Phase 1: copy from `tiffany/deployment-configuration/profiles/dev/resurrection-node/`.

### 8. Hub registration

```
POST ${REGENESIS_HUB_URL}/v2/register
{
  "node_id": "...",
  "capacity": { "vcpu": N, "memory_mb": M },
  "labels": { "site": "ms02-lab" }
}
```

Receive mTLS cert + CA (or confirm pre-installed cert).

Start `cylon-host.service`.

### 9. Health verification

```
curl -sf http://127.0.0.1:8080/health
grpc health (optional)
```

### 10. Finalize

```
touch /var/lib/regenesis/configured
emit DCops lifecycle event: installed (Phase 2 API)
```

Operator commits BootIntent `lifecycle: locked`.

## Idempotency rules

| Step | Re-run behavior |
|---|---|
| Packages | apt idempotent |
| Firecracker | skip if version matches |
| Guest kernel | skip if sha256 matches |
| Cylon binary | skip if release pin unchanged |
| Register | PUT if node already known |

## Failure handling

| Failure | Action |
|---|---|
| GHCR pull fail | Retry 3×; leave marker absent; systemd retry |
| Release download fail | Exit 1; do not start cylon-host |
| Hub register fail | Exit 1; log; retry oneshot in 60s |
| Partial state | No `/var/lib/regenesis/configured` — safe to re-run |

## Observability

- Structured logs to journald `SYSLOG_IDENTIFIER=regenesis-agent`
- Span: `regenesis.first_boot` exported if `OTEL_EXPORTER_OTLP_ENDPOINT` set

## Implementation plan

| Milestone | Deliverable |
|---|---|
| Phase 1a | Shell script parity in `scripts/regenesis-agent.sh` |
| Phase 1b | Rust CLI `crates/regenesis-agent` |
| Phase 2 | DCops lifecycle callback |

## References

- [cloud-init-parity.md](cloud-init-parity.md)
- `cylon-images/multipass/cloud-init.yaml`
- `tiffany/deployment-configuration/profiles/dev/resurrection-node/`
