# DCops integration

How **cylon-regenesis** uses [DCops](../../../DCops/) for datacenter GitOps — without Tinkerbell.

## Division of responsibility

| Concern | DCops | cylon-regenesis |
|---|---|---|
| MAC → boot profile mapping | `BootIntent` controller | Profile content + iPXE scripts |
| Kernel/initrd URLs | `BootProfile` CRD | Build + publish artifacts to pxe-server volume |
| Management IP | `IPPool` + `IPClaim` | Document pool names; hub uses registered IP |
| Device inventory | NetBox CRDs | Optional `NetBoxDevice` per rack unit |
| DHCP | Kea deployment | Option 66/67 values pointing at pxe-server |
| HTTP iPXE serve | `pxe-server` crate | File layout under `/cylon-regenesis/` |
| Hub registration | — | `regenesis-agent` |
| Boot lifecycle lock | `BootIntent.status.lifecycle` | Agent triggers `installed`/`locked` |

## DCops components used

### CRDs (group `dcops.microscaler.io/v1alpha1`)

| CRD | Purpose for CRP |
|---|---|
| `BootProfile` | kernel, initrd[], cmdline for iPXE |
| `BootIntent` | MAC → profile + lifecycle |
| `IPPool` / `IPClaim` | Resurrection node management IP |
| `NetBoxDevice` | Rack asset tracking (optional) |
| `NetBoxMACAddress` | Validate MAC before boot allowed |

Source: `DCops/crates/crds/src/boot_profile.rs`, `boot_intent.rs`

### pxe-server

Rust crate providing:

- DHCP integration (with Kea or embedded — see DCops roadmap)
- **HTTP server for iPXE** — preferred over TFTP for speed (`crates/pxe-server/src/http.rs`)

cylon-regenesis publishes static boot artifacts to the volume mounted into pxe-server (ConfigMap, PVC, or object storage sync — TBD in Phase 2).

### Kea DHCP

`DCops/config/kea-dhcp/` — DHCP server for lab/production VLAN.

Required options for iPXE:

```
option 66: ${pxe_server_ip}   # next-server
option 67: ipxe.efi            # or snponly-undionly.kpxe
```

iPXE DHCP user class may chain to HTTP script URL directly (site-specific).

## GitOps workflow

```
1. Engineer commits BootProfile + BootIntent YAML to DCops Git repo
2. DCops controllers reconcile → NetBox + pxe-server config
3. Bare metal PXE boots → regenesis-agent → hub register
4. Engineer commits lifecycle: locked
```

## Namespace layout (recommended)

| Namespace | Contents |
|---|---|
| `dcops-system` | Controllers, pxe-server, Kea |
| `cylon-regenesis` | BootProfile, BootIntent for resurrection fleet |
| `netbox` | NetBox instance |

## Example kustomize patch

Reference only — lives in DCops or cylon-regenesis `deploy/` Phase 2:

```yaml
# cylon-regenesis BootIntent — see ipxe-provisioning.md for full spec
apiVersion: dcops.microscaler.io/v1alpha1
kind: BootIntent
metadata:
  name: ms02-lab-resurrection-01
  namespace: cylon-regenesis
spec:
  macAddress: "${LAB_MAC}"
  profileRef:
    name: cylon-resurrection-ubuntu-2404
  lifecycle: discovered
```

## Interaction with Multipass dev

Multipass VMs **do not** use DCops in Phase 1. MAC addresses are synthetic — BootIntent would pollute NetBox.

Cutover criteria ([phase-2](../phases/phase-2-dcops-ipxe-dev.md)):

- regenesis-agent proven on Multipass
- One lab bare-metal MAC in NetBox
- pxe-server HTTP serves test profile

## Failure modes

| Failure | Detection | Response |
|---|---|---|
| Wrong BootProfile | Autoinstall wrong OS | BootIntent validation webhook (future) |
| Infinite PXE loop | lifecycle stuck `installing` | Watchdog alert; manual lifecycle reset |
| IP conflict | IPClaim status error | DCops drift detection |
| pxe-server down | DHCP OK but HTTP timeout | iPXE retry; alert on Kea logs |

## References

- DCops [README](../../../DCops/README.md)
- DCops [ADR-001](../../../DCops/ADRs/ADR-001-Scope_and_Non-Goals.md)
- [ipxe-provisioning.md](ipxe-provisioning.md)
