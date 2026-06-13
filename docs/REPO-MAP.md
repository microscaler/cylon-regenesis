# Repository map

Sibling repos under `~/Workspace/microscaler/` (ms02 NFS) and how they connect to **cylon-regenesis**.

## Dependency graph

```
                    cylon (portal, engine, cylon host)
                         │
           ┌─────────────┼─────────────┐
           ▼             ▼             ▼
   cylon-regenesis  cylon-images   cylon-local-infra
   (hub, regenesis)  (guest GHCR)   (Tilt, Kind ops)
           │
           ▼
        DCops (iPXE, IPAM, Kea, NetBox)
```

## Per-repo contract

### cylon-regenesis (this repo)

| Owns | Does not own |
|---|---|
| regenesis-hub (future) | `CylonService` implementation |
| regenesis-agent | Guest OCI build |
| iPXE scripts + host OS image spec | Platform Postgres / portal |
| Hub `/v2` OpenAPI spec | Talos/CAPI |

**Consumers:** cylon (spawns agents via hub URL), DCops (serves boot files).

---

### cylon

| Path | Role |
|---|---|
| `crates/cylon/` | Host daemon — stays until optional future split |
| `crates/resurrection-hub/` | **Migrate out** to regenesis-hub Phase 3 |
| `crates/engine/` | Agent LLM loop |
| `deployment-configuration/profiles/dev/resurrection-node/` | Per-node env (until regenesis-agent embeds) |
| `justfile` `resurrection-nodes-*` | Dev ops — calls hub + Multipass |

**Integration point:** Hub URL `http://resurrection-hub:14000` (Kind) → becomes regenesis-hub image from this repo.

---

### cylon-images

| Path | Role |
|---|---|
| `container/kernel/` | Guest `vmlinux` CI → GHCR |
| `container/rootfs/` | Guest OCI CI → GHCR |
| `multipass/` | **Dev-only** host provisioning until Phase 2 cutover |

**Integration point:** GHCR refs passed in `CreateCylonVm.container_source`. Host kernel install via `install-kernel-from-ghcr` / regenesis-agent.

---

### DCops

| Path | Role |
|---|---|
| `crates/crds/` | `BootIntent`, `BootProfile`, `IPClaim`, … |
| `crates/pxe-server/` | DHCP assist + **iPXE HTTP** |
| `config/kea-dhcp/` | DHCP server (Phase 2+) |
| `config/netbox/` | IPAM backend |

**Integration point:** cylon-regenesis publishes BootProfile YAML examples under `ipxe/profiles/` consumed by DCops GitOps.

**Important:** DCops ADR-001 targets Pi/Talos for PriceWhisperer. CRP resurrection profiles are an **additional BootProfile use case** on x86_64 — no change to DCops non-goals.

---

### cylon-local-infra

| Role |
|---|
| Kind cluster, Tilt, resurrection-node Ansible |
| Documents: `docs/far-multipass.md` |

**Integration point:** Phase 2 adds DCops + regenesis boot to lab topology doc.

---

### liquidmetal/flintlock (reference only)

| Path | Role |
|---|---|
| `docs/proposals/distributed_architecture/` | Requirements baseline — not implemented upstream |

## Version pinning flow

```
cylon-regenesis release tag
  ├── pins regenesis-agent version
  ├── pins minimum cylon host release (cylon tag)
  ├── pins BootProfile image digest
  └── documents GHCR guest tags (from cylon-images CI)
```

## Where to change what

| Task | Repo |
|---|---|
| Fix Firecracker boot args | cylon `crates/cylon` |
| Rebuild guest rootfs | cylon-images |
| Add hub scheduler feature | cylon-regenesis (after Phase 3) |
| New resurrection rack IP pool | DCops IPPool CRD |
| New iPXE script | cylon-regenesis `ipxe/` |
| Portal spawn agent UX | Cylon ui + hub client |
