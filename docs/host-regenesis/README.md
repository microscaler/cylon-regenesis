# Host regenesis

**Goal:** Turn bare metal into a **registered resurrection node** ready for Firecracker agent scheduling.

## Pipeline

```
DCops BootIntent (MAC)
    → iPXE HTTP boot
    → Host OS installed (Ubuntu 24.04 LTS minimal + KVM)
    → regenesis-agent first boot
    → POST /v2/register (regenesis-hub)
    → BootIntent.lifecycle = locked
```

## Documents

| Doc | Content |
|---|---|
| [ipxe-provisioning.md](ipxe-provisioning.md) | iPXE scripts, DHCP, HTTP paths |
| [dcops-integration.md](dcops-integration.md) | CRDs, NetBox, Kea |
| [first-boot-sequence.md](first-boot-sequence.md) | regenesis-agent step-by-step |
| [cloud-init-parity.md](cloud-init-parity.md) | Multipass equivalence |

## Dev vs prod

| | Dev (Phase 1) | Prod (Phase 2+) |
|---|---|---|
| Provisioner | Multipass cloud-init | DCops iPXE |
| Agent | regenesis-agent binary | same |
| Hub | Kind :14000 | HA regenesis-hub |

## Non-goals

- Provisioning **guest** microVM rootfs (hub pulls GHCR at schedule time).
- Talos / Kubernetes node join.
