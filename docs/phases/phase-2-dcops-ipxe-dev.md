# Phase 2 — DCops iPXE lab

**Depends on:** Phase 1 (regenesis-agent proven)  
**Goal:** One **lab bare-metal** x86_64 host provisions exclusively via DCops standard iPXE.

## Prerequisites

| Prerequisite | Owner |
|---|---|
| DCops on shared Kind cluster (`kind-kind`) on ms02 | DCops — `just dev-up` in `DCops/` |
| pxe-server HTTP serving boot files | DCops `crates/pxe-server` |
| Kea DHCP on lab VLAN OR DHCP relay | DCops Phase 2 |
| NetBox MAC for lab server | DCops |
| regenesis-agent in autoinstall user-data | cylon-regenesis |

## Work items

### 2.1 Host OS netboot image

Build Ubuntu 24.04 autoinstall netboot artifacts:

```
cylon-regenesis/images/host-os/ubuntu-24.04/
├── build.sh (or Makefile — no shell scripts policy in cylon; use just)
├── autoinstall/user-data
└── publish → pxe-server PVC
```

user-data invokes regenesis-agent with `REGENESIS_HUB_URL=http://192.168.1.189:14000` (ms02 Kind forward).

### 2.2 iPXE scripts

- Commit `ipxe/cylon-resurrection.ipxe`
- Sync to pxe-server volume (InitContainer or manual rsync Phase 2a)

### 2.3 DCops CRDs

Apply in namespace `cylon-regenesis`:

- `BootProfile` `cylon-resurrection-ubuntu-2404`
- `BootIntent` for lab MAC
- `IPClaim` for management IP

See [dcops-integration.md](../host-regenesis/dcops-integration.md).

### 2.4 pxe-server HTTP completion

DCops `HttpServer` is currently `todo!()` — **blocker**.

Options:

| Option | Effort |
|---|---|
| Complete DCops `pxe-server` HTTP | Preferred — aligns ADR-0003 |
| Temporary nginx sidecar serving `/cylon-regenesis/` | Lab unblock only |

Track in DCops repo; cylon-regenesis documents required HTTP paths.

### 2.5 End-to-end test

1. Wipe lab disk
2. PXE boot
3. Autoinstall + regenesis-agent
4. Hub shows node `Online`
5. BootIntent `locked`
6. `CreateCylonVm` smoke from hub

## Acceptance criteria

- [ ] Zero Multipass steps for lab server
- [ ] BootIntent lifecycle reaches `locked`
- [ ] Re-PXE boot does not reinstall
- [ ] Time to registered <15 minutes

## Rollback

- Boot local disk if present
- Set BootIntent `discovered` only with ops approval

## Next phase

[Phase 3 — Control plane extraction](phase-3-control-plane-extraction.md)
