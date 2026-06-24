# 07 — iPXE + DCops specification

Standard **iPXE** only. **DCops** orchestrates DHCP, HTTP, and GitOps intent. **Not Tinkerbell.**

---

## Physical topology (target lab)

```
┌─────────────────────────────────────────────────────────┐
│ Management VLAN (192.168.1.0/24 example)                 │
│  ┌──────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │ Kea DHCP │  │ DCops       │  │ Kind: regenesis-hub │ │
│  │          │──│ pxe-server  │  │ :14000              │ │
│  └────┬─────┘  │ HTTP :8080  │  └─────────────────────┘ │
│       │        └──────▲──────┘                           │
│       │               │ PVC: pxe-artifacts               │
│  ┌────┴───────────────┴──────────────────────────────┐  │
│  │ Bare metal resurrection node (PXE client)           │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

ms02 dev: hub on Kind; DCops (NetBox, Kea, pxe-server) on shared `kind-kind`; lab server same L2.

---

## DHCP requirements

### Kea options (iPXE)

| Option | Value |
|---|---|
| 66 (`next-server`) | pxe-server Service ClusterIP or LB IP |
| 67 (`boot-file-name`) | `ipxe.efi` / `snponly.undionly.kpxe` |

### iPXE DHCP user class (preferred)

If client sends `iPXE` user class, Kea returns:

```
filename: http://${next-server}/ipxe/cylon-resurrection.ipxe
```

Avoids chainload UNDI then script fetch twice.

---

## HTTP URL namespace

Base: `http://{pxe-server}/`

| Path | Content-Type | Description |
|---|---|---|
| `/ipxe/cylon-resurrection.ipxe` | text/plain | Entry script |
| `/cylon-regenesis/profiles/{profile}/vmlinuz` | application/octet-stream | Host kernel |
| `/cylon-regenesis/profiles/{profile}/initrd.img` | application/octet-stream | Initrd |
| `/cylon-regenesis/profiles/{profile}/autoinstall/meta-data` | text/plain | cloud-init meta |
| `/cylon-regenesis/profiles/{profile}/autoinstall/user-data` | text/yaml | autoinstall + regenesis |
| `/cylon-regenesis/regenesis-agent/{version}/regenesis-agent` | binary | First-boot binary |
| `/ipxe/ipxe.efi` | binary | iPXE binary (optional mirror) |

---

## Profile: `ubuntu-24.04-autoinstall`

### BootProfile CRD

```yaml
apiVersion: dcops.microscaler.io/v1alpha1
kind: BootProfile
metadata:
  name: cylon-resurrection-ubuntu-2404
  namespace: cylon-regenesis
spec:
  kernel: http://pxe.mgmt/cylon-regenesis/profiles/ubuntu-24.04-autoinstall/vmlinuz
  initrd:
    - http://pxe.mgmt/cylon-regenesis/profiles/ubuntu-24.04-autoinstall/initrd.img
  cmdline: >-
    ip=dhcp autoinstall ds=nocloud-net;s=http://pxe.mgmt/cylon-regenesis/profiles/ubuntu-24.04-autoinstall/autoinstall/
    regenesis.profile=ubuntu-24.04-autoinstall
    regenesis.hub=http://192.168.1.189:14000
  message: Cylon resurrection node — Ubuntu 24.04 autoinstall
```

### user-data (excerpt)

```yaml
#cloud-config
autoinstall:
  version: 1
  identity:
    hostname: resurrection-node
    username: cylon
    password: "$6$..."  # or ssh keys only
  ssh:
    install-server: true
  storage:
    layout:
      name: direct
  packages:
    - openssh-server
  late-commands:
    - curtin in-target -- wget -O /usr/local/bin/regenesis-agent http://pxe.mgmt/cylon-regenesis/regenesis-agent/0.1.0/regenesis-agent
    - curtin in-target -- chmod +x /usr/local/bin/regenesis-agent
    - curtin in-target -- mkdir -p /etc/regenesis
    - curtin in-target -- wget -O /etc/regenesis/config.yaml http://pxe.mgmt/cylon-regenesis/profiles/ubuntu-24.04-autoinstall/regenesis-config.yaml
    - curtin in-target -- systemctl enable regenesis-agent
```

Token injection: **not** in user-data Git — fetched at runtime from `/etc/cylon/github-token` installed by sealed secret side-channel or manual first boot (Phase 2a).

---

## BootIntent lifecycle controller behavior

| Event | Controller action |
|---|---|
| CRD created `discovered` | Validate MAC in NetBox |
| PXE request observed | Set `installing` (optional) |
| regenesis callback `installed` | Set `installed` (Phase 2 API) |
| Operator commits `locked` | pxe-server denies reinstall |

### pxe-server lookup algorithm

```
1. Extract client MAC from DHCP relay or HTTP header (X-Client-MAC future)
2. List BootIntent where spec.macAddress == MAC
3. If none → HTTP 404 "unknown host"
4. If lifecycle == locked → serve localboot.ipxe only
5. Resolve BootProfile from profileRef
6. Render iPXE script with kernel/initrd/cmdline
```

**Gap today:** DCops pxe-server HTTP not implemented — [02-gap-analysis](02-gap-analysis-current-state.md).

---

## iPXE scripts

### Entry: `cylon-resurrection.ipxe`

See repo `ipxe/cylon-resurrection.ipxe`.

### Local boot only: `localboot.ipxe`

```ipxe
#!ipxe
echo BootIntent locked — local disk only
sanboot --no-describe --drive 0x80
```

---

## regenesis-config.yaml per node (HTTP served)

Generated from template + BootIntent metadata:

```yaml
node_id: ms02-lab-u42
hub_api: http://192.168.1.189:14000
grpc_endpoint: https://192.168.1.50:50052
# ... see 06-regenesis-agent-spec.md
labels:
  boot_intent: ms02-lab-u42
  site: ms02
```

IP for grpc_endpoint comes from NetBox IPClaim status.

---

## DCops dependency matrix

| DCops component | Maturity | CRP need |
|---|---|---|
| BootProfile CRD | ✅ defined | Use as-is |
| BootIntent CRD | ✅ defined | Use as-is |
| IPClaim | ✅ | Management IP |
| pxe-server HTTP | ❌ todo | **Blocker** |
| Kea deployment | 🟡 config exists | DHCP options |
| NetBox MAC CRD | 🟡 partial reconcile | Inventory |

---

## Lab workaround (Phase 2a)

Until DCops HTTP complete:

1. nginx Deployment mounting hostPath `~/Workspace/microscaler/cylon-regenesis/artifacts/pxe`
2. Kea next-server → nginx ClusterIP
3. Manual BootIntent apply

Document in runbook — not production.

---

## Security

| Layer | Phase 2 lab | Production |
|---|---|---|
| PXE HTTP | isolated VLAN | HTTPS or TLS proxy |
| user-data secrets | none in Git | vault agent on node |
| BootIntent lock | manual GitOps | required before prod traffic |

---

## Acceptance test script

```
1. Apply BootProfile + BootIntent + IPClaim
2. Power on lab server (PXE boot)
3. Observe autoinstall progress (IPMI serial or KVM)
4. regenesis-agent completes; /var/lib/regenesis/configured exists
5. curl hub /v2/register reflected in GET agents test
6. Commit BootIntent lifecycle: locked
7. Reboot → no reinstall
```

Epics: **REG-PXE-***, **REG-DCOPS-*** in WBS.
