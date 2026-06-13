# 02 — Gap analysis: current state

**As of 2026-06-13** — audited against `tiffany/crates/resurrection-hub`, `tiffany/crates/cylon`, `cylon-images/multipass/`, `DCops/`.

Legend: ✅ implemented | 🟡 partial | ❌ missing | 📋 spec only

## Control plane (`resurrection-hub` → future `regenesis-hub`)

| Capability | Flintlock # | Status | Location / notes |
|---|---|---|---|
| OpenRaft cluster | 01 | ✅ | `resurrection-hub/src/raft/`, snapshot every 10k logs |
| HTTP `/v2/register` | 03 | ✅ | `api.rs` — also used as heartbeat |
| HTTP `/v2/agents` POST | 02 | ✅ | → allocator → gRPC CreateCylonVm |
| HTTP `/v2/agents/{id}` GET proxy | 03 | ✅ | GetCylonVm to authoritative node |
| Batch allocator 50ms | 08 | ✅ | `allocator.rs` — max 500/batch |
| `/v2/nodes/rejoin` | 09 | ✅ | kill/keep lists |
| `/v2/nodes/{id}/drain` | 14 | 🟡 | Calls MigrateCylonVm; migration pipe stub on host |
| Hub heartbeat monitor | 04 | ✅ | Leader marks Offline @15s; reschedule loop in `main.rs` |
| OTEL inject to gRPC | 13 | ✅ | `MetadataInjector` in api.rs |
| S3 CryoSleep GC | 12 | ❌ | PRD marked done; no background sweep in code |
| Formal OpenAPI | 15 | ❌ | Routes exist; no published schema |
| Typed Raft commands | 01 | 🟡 | JSON blobs in `client_status` map |
| `GET /v2/nodes` | 03 | ❌ | Not exposed |
| `POST /v2/agents/{id}/hibernate` | — | ❌ | Only via gRPC direct |
| Auth on HTTP API | 11 | ❌ | No middleware; dev only |
| 3-node HA deployment | 01 | 🟡 | Raft routes exist; Kind runs single pod |

## Cylon host (`tiffany/crates/cylon`)

| Capability | Status | Notes |
|---|---|---|
| CreateCylonVm OCI→ext4 | ✅ | `oci.rs` — anonymous GHCR; :5001 HTTP |
| Firecracker lifecycle | ✅ | `lifecycle.rs`, `client.rs` |
| Hibernate / Resurrect RPC | 🟡 | Proto defined; impl depth varies |
| MigrateCylonVm | 🟡 | API wired; memory pipe incomplete |
| Detached watchdog | ✅ | 5s register ping; detach @30s; pause VMs |
| Rejoin kill list | 🟡 | Removes UDS file only — no full DeleteCylonVm |
| Hub mTLS server | ✅ | gRPC with certs |
| OCI GHCR auth | ❌ | Anonymous only |
| Metrics `/metrics` | ❌ | Health only `:8080` |
| Bid RPC to hub | ❌ | Hub pulls capacity from register payload |

## Host regenesis

| Capability | Status | Notes |
|---|---|---|
| Multipass cloud-init | ✅ | `cylon-images/multipass/` |
| GHCR guest kernel in cloud-init | ✅ | crane install-kernel-from-ghcr |
| regenesis-agent | ❌ | Documented only |
| iPXE boot | 📋 | `ipxe/cylon-resurrection.ipxe` stub |
| DCops BootProfile for CRP | ❌ | DCops targets Pi/Talos |
| DCops pxe-server HTTP | ❌ | `todo!()` in http.rs |
| BootIntent lifecycle API | ❌ | Manual GitOps only |

## Guest artifacts (`cylon-images`)

| Artifact | Status |
|---|---|
| GHCR `cylon-kernel:6.1.102` | ✅ published |
| GHCR `cylon-rootfs-ubuntu:latest` | ✅ published |
| Minimal dev rootfs `:5001` | ✅ ms02 |

## Documentation (`cylon-regenesis`)

| Item | Status |
|---|---|
| ADRs 0001–0005 | ✅ |
| ARCHITECTURE, PRD, phases | ✅ |
| This master plan | ✅ in progress |

## Priority order (implementation)

1. **P0** — regenesis-agent + Multipass parity (unblocks self-registering dev fleet)
2. **P1** — Hub crate extraction to cylon-regenesis (ownership boundary)
3. **P2** — DCops pxe-server HTTP + CRP BootProfile (unblocks bare metal)
4. **P3** — Harden gaps: S3 GC, rejoin DeleteCylonVm, GHCR auth, OpenAPI
5. **P4** — Production HA + chaos certification

## Risk register

| Risk | Impact | Mitigation |
|---|---|---|
| DCops HTTP PXE delayed | Blocks Phase 2 | nginx sidecar lab workaround |
| Raft JSON state machine | Schema drift | Phase 3 refactor to typed commands |
| rejoin only deletes UDS | Ghost FC processes | Epic REG-CP-4.3 |
| Private GHCR on nodes | CreateCylonVm fails | Epic REG-HOST-3.1 |
| Hub single replica Kind | No HA dev | Accept Phase 1–3; 3× Phase 5 |
