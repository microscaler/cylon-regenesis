# 06 — regenesis-agent specification

**Component:** `regenesis-agent` (future: `crates/regenesis-agent`)  
**Role:** Idempotent first-boot provisioner for resurrection nodes  
**Replaces:** `cylon-images/multipass/cloud-init.yaml` runcmd subset + manual `resurrection-nodes-deploy-host-daemon`

---

## Invocation

| Environment | Trigger |
|---|---|
| Multipass Phase 1 | `systemd` oneshot `regenesis-agent.service` After=`network-online.target` |
| iPXE Phase 2 | autoinstall `late-commands` or first-boot systemd |
| Manual | `sudo regenesis-agent --config /etc/regenesis/config.yaml` |

### CLI (target)

```
regenesis-agent [OPTIONS]

Options:
  --config PATH          YAML config (default /etc/regenesis/config.yaml)
  --dry-run              Print actions, no mutations
  --phase PHASE          Run single phase (preflight|packages|firecracker|...)
  --force                Ignore configured marker
  -v, --verbose
```

---

## Configuration file

`/etc/regenesis/config.yaml` (mode 0644 root:root — no secrets here)

```yaml
node_id: resurrection-node-1
hub_api: http://192.168.1.189:14000
grpc_endpoint: https://192.168.1.50:50052
grpc_listen: 0.0.0.0:50052

cylon:
  release_pin: latest          # or v0.1.1
  github_token_file: /etc/cylon/github-token

guest_kernel:
  image: ghcr.io/microscaler/cylon-kernel:6.1.102
  dest: /home/cylon/cylon-images/vmlinux

firecracker:
  version: 1.10.1

capacity:
  memory_mb: 8192              # 0 = auto-detect from /proc/meminfo
  vcpu: 8                      # 0 = auto-detect

labels:
  site: ms02
  provisioning: multipass

dcops:
  boot_intent_name: ""         # optional callback target
  lifecycle_callback_url: ""   # Phase 2
```

Secrets **never** in config.yaml:

- `/etc/cylon/github-token` (0600)
- mTLS keys in `/etc/cylon/certs/` (0700)

---

## Phase machine (internal)

```
START → preflight → packages → user_kvm → firecracker → guest_kernel
     → cylon_binary → systemd → certs → register → health → finalize → END
```

Each phase:

1. Logs `regenesis.phase.start name={phase}`
2. Checks phase cache in `/var/lib/regenesis/phases/{phase}.done`
3. Executes work
4. Writes phase marker on success
5. On failure: exit code ≠ 0, systemd retry

Global marker: `/var/lib/regenesis/configured` — if present and not `--force`, exit 0 immediately.

---

## Phase: preflight

| Check | Fail message |
|---|---|
| EUID root or passwordless sudo for writes | `EUID` |
| `/dev/kvm` exists | `NO_KVM` |
| `curl --version` | `NO_CURL` |
| Default route | `NO_NETWORK` |
| `hub_api` TCP connect (3s timeout) | `HUB_UNREACHABLE` warn-only Phase 1 |

---

## Phase: packages

```bash
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y --no-install-recommends \
  qemu-kvm cpu-checker acl curl jq iproute2 ca-certificates libssl3 crane
```

Verify: `crane version`, `kvm-ok` (warn if fail on nested KVM).

---

## Phase: user_kvm

```bash
id cylon || useradd -m -s /bin/bash -G sudo,kvm cylon
echo 'cylon ALL=(ALL) NOPASSWD:ALL' > /etc/sudoers.d/cylon
setfacl -m u:cylon:rw /dev/kvm
mkdir -p /home/cylon/cylon-images
chown -R cylon:cylon /home/cylon
```

---

## Phase: firecracker

Download pinned tarball:

```
https://github.com/firecracker-microvm/firecracker/releases/download/v1.10.1/firecracker-v1.10.1-x86_64.tgz
```

Install:

- `/usr/bin/firecracker`
- `/usr/bin/jailer`

Verify: `/usr/bin/firecracker --version` contains `1.10.1`

Skip if existing version matches.

---

## Phase: guest_kernel

```bash
TOKEN=""
[ -f /etc/cylon/github-token ] && crane auth login ghcr.io -u token -p "$(tr -d '[:space:]' < /etc/cylon/github-token)"
crane export ghcr.io/microscaler/cylon-kernel:6.1.102 - | tar -xOf - vmlinux > /tmp/vmlinux.new
mv /tmp/vmlinux.new /home/cylon/cylon-images/vmlinux
chown cylon:cylon /home/cylon/cylon-images/vmlinux
chmod 644 /home/cylon/cylon-images/vmlinux
```

Verify: file size > 1MB, `file` reports ELF.

Skip if sha256 matches `/var/lib/regenesis/guest_kernel.sha256`.

---

## Phase: cylon_binary

Port of `install-cylon-from-release` from cloud-init:

1. Read `release_pin` from config
2. GitHub API `microscaler/tiffany` releases
3. Download `cylon-linux-x86_64` + `.sha256` via API asset URLs
4. Verify hash
5. `install -m 755 → /usr/local/bin/cylon`

Fail codes: `TOKEN_MISSING`, `RELEASE_NOT_FOUND`, `CHECKSUM_MISMATCH`

---

## Phase: systemd

Install files:

| File | Source (Phase 1) |
|---|---|
| `/etc/systemd/system/cylon-host.service` | embed template or copy from tiffany deployment-configuration |
| `/etc/cylon/host.env` | generated from config |

Template `host.env`:

```bash
CYLON_NODE_ID=resurrection-node-1
CYLON_GRPC_ENDPOINT=https://10.0.0.50:50052
HUB_API_ENDPOINT=http://192.168.1.189:14000
CYLON_AVAILABLE_MEMORY_MB=8192
CYLON_AVAILABLE_VCPU=8
CYLON_CERTS_DIR=/etc/cylon/certs
```

```bash
systemctl daemon-reload
systemctl enable cylon-host.service
systemctl start cylon-host.service
```

---

## Phase: certs

Phase 1 (Multipass): copy from tiffany decrypted bundle via existing `just resurrection-nodes-deploy-host-daemon` **or** embed dev certs in cloud-init merge.

Phase 2+: hub registration returns cert bundle (future) OR pre-seeded per-site CA.

Required files in `CYLON_CERTS_DIR`:

- `ca.crt`
- `server.crt`, `server.key` (node gRPC server)
- optional client certs for hub dial

---

## Phase: register

```bash
curl -sf -X POST "${hub_api}/v2/register" \
  -H 'Content-Type: application/json' \
  -d @/tmp/regenesis-register.json
```

Payload from [04-api-contract.md](04-api-contract.md).

Retry: 5 attempts, exponential backoff 2s→32s.

---

## Phase: health

Wait up to 60s:

```bash
curl -sf http://127.0.0.1:8080/health
```

Optional: grpcurl `GetCylonVm` missing VM → NotFound OK.

---

## Phase: finalize

```bash
date -Iseconds > /var/lib/regenesis/configured
regenesis-agent-version > /var/lib/regenesis/version
```

Phase 2: POST lifecycle `installed` to DCops callback (TBD).

---

## systemd unit (regenesis-agent)

```ini
[Unit]
Description=Cylon Regenesis first-boot agent
After=network-online.target
Wants=network-online.target
ConditionPathExists=!/var/lib/regenesis/configured

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/usr/local/bin/regenesis-agent --config /etc/regenesis/config.yaml
StandardOutput=journal
StandardError=journal
SyslogIdentifier=regenesis-agent

[Install]
WantedBy=multi-user.target
```

---

## Exit codes

| Code | Meaning |
|---|---|
| 0 | Success or already configured |
| 1 | Generic failure |
| 10 | preflight |
| 20 | packages |
| 30 | firecracker |
| 40 | guest_kernel |
| 50 | cylon_binary |
| 60 | systemd |
| 70 | register |
| 80 | health |

---

## Testing requirements

| Test | Type |
|---|---|
| Idempotent second run | integration |
| Missing token | unit |
| Bad checksum | unit |
| Hub down at register | integration warn/retry |
| Full Multipass E2E | manual ms02 |

Epic: **REG-AGENT-*** in [13-work-breakdown-structure.md](13-work-breakdown-structure.md).

---

## Multipass integration diff (Phase 1)

Replace cloud-init runcmd lines 111–117 with:

```yaml
runcmd:
  - ... firecracker install (or delegate all to agent) ...
  - curl -fsSL -o /usr/local/bin/regenesis-agent ${REGENESIS_AGENT_URL}
  - chmod +x /usr/local/bin/regenesis-agent
  - install -D -m 644 /opt/regenesis/config.yaml /etc/regenesis/config.yaml
  - systemctl enable regenesis-agent.service
  - systemctl start regenesis-agent.service
```

Keep `install-cylon-from-release` removal — agent subsumes it.
