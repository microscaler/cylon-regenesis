# ADR-0005: Multipass Dev → Bare-Metal Production

## Status

**Accepted** — 2026-06-13

## Context

ms02 development uses **Multipass** resurrection nodes (`cylon-images/multipass/cloud-init.yaml`) — fast iteration, nested KVM, no datacenter hardware. Production requires **bare-metal resurrection nodes** on the DCops-managed network with iPXE regenesis.

## Decision

### Phase ladder

| Phase | Environment | Provisioning | Hub |
|---|---|---|---|
| **Now** | ms02 Multipass `resurrection-node-{1,2,3}` | cloud-init merge + GitHub Releases | Kind `resurrection-hub` |
| **1** | Multipass + **regenesis-agent** binary | Same cloud-init semantics, agent extracted to regenesis repo | unchanged |
| **2** | Lab bare metal (single rack unit) | DCops BootIntent + iPXE | Kind hub |
| **3** | Production DC | Full regenesis loop, locked BootIntent | HA regenesis-hub cluster |

### Parity requirement

`regenesis-agent` **must** produce equivalent host state to current cloud-init:

- User `cylon`, group `kvm`, `/dev/kvm` ACL
- Firecracker 1.10.x + jailer
- `/home/cylon/cylon-images/vmlinux` from **GHCR** (`cylon-kernel:6.1.102`)
- `/usr/local/bin/cylon` from Cylon GitHub Releases
- `cylon-host.service` + `/etc/cylon/host.env`
- mTLS client material for Hub gRPC

Documented in [cloud-init-parity.md](../docs/host-regenesis/cloud-init-parity.md).

### What Multipass is not

- Not a production hypervisor tier.
- Not a substitute for DCops IPAM — Multipass uses bridged DHCP from the Mac/ms02 LAN.

## Consequences

- Two provisioning paths maintained until Phase 3 cutover.
- iPXE testing requires DCops on the shared Kind cluster on ms02 (`kind-kind`; see `DCops/AGENTS.md`).

## References

- [phase-1-multipass-parity.md](../docs/phases/phase-1-multipass-parity.md)
- [phase-2-dcops-ipxe-dev.md](../docs/phases/phase-2-dcops-ipxe-dev.md)
