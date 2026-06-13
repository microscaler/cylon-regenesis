# ADR-0003: Standard iPXE via DCops — Not Tinkerbell

## Status

**Accepted** — 2026-06-13

## Context

Flintlock proposal doc **06** describes PXE-based host regenesis with immutable OS images. Microscaler has:

- **DCops** (`~/Workspace/microscaler/DCops/`) — GitOps datacenter control: `BootIntent`, `BootProfile`, IP claims, Kea DHCP, Rust `pxe-server` with **iPXE over HTTP**.
- **Tinkerbell** expertise (founder is Tinkerbell lead engineer) — deliberately **not** used for CRP resurrection nodes to avoid workflow/Hook complexity and keep one operational stack.

Current dev path uses **Multipass + cloud-init** in `cylon-images/multipass/` — adequate for ms02 lab, not production bare metal.

## Decision

1. **Production host provisioning** uses **standard iPXE** (chainload → HTTP script → kernel/initrd or netboot installer).
2. **DCops** is the orchestration layer:
   - `BootProfile` — kernel URL, initrd[], cmdline, iPXE script template reference.
   - `BootIntent` — MAC → profile mapping, lifecycle (`discovered` → `installing` → `installed` → `locked`).
   - `IPClaim` / NetBox — deterministic resurrection-node management IP.
   - `pxe-server` crate — DHCP helper + **HTTP delivery for iPXE** (dual-stack target).
3. **cylon-regenesis** owns:
   - **`RegenesisBootProfile`** CRD (or DCops profile content) for Cylon host OS images.
   - **`regenesis-agent`** first-boot binary/script — installs Firecracker, Cylon host, GHCR guest kernel, registers with Hub.
   - Hub **rejoin handshake** after reprovision.
4. **Not in scope:** Tinkerbell Workflow, Smee, Hook OS, Hegel gRPC.

### iPXE boot chain (resurrection node)

```
Bare metal powers on
  → DHCP (Kea / DCops) offers next-server + filename (iPXE UNDI)
  → iPXE loads http://pxe.<site>/cylon-resurrection.ipxe
  → Script: kernel + initrd OR live installer
  → First boot: regenesis-agent (cloud-init parity)
  → POST /v2/register + mTLS certs
  → BootIntent.lifecycle = installed → locked
```

## Consequences

- DCops Phase 1 PXE must support x86_64 resurrection hardware (DCops ADR-001 targets Pi/Talos first — **CRP profiles are an additional BootProfile use case**, not a DCops scope expansion).
- cylon-regenesis documents exact BootProfile YAML and iPXE scripts; DCops controllers reconcile them.
- Multipass remains dev-only until Phase 2 ([ADR-0005](ADR-0005-dev-multipass-to-bare-metal.md)).

## References

- [dcops-integration.md](../docs/host-regenesis/dcops-integration.md)
- [ipxe-provisioning.md](../docs/host-regenesis/ipxe-provisioning.md)
- DCops `crates/crds/src/boot_profile.rs`, `crates/pxe-server/`
