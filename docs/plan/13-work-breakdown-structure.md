# 13 — Work breakdown structure

Task IDs: `{EPIC}-{nn}` — use in issues/PRs.

Estimates: **S** ≤2d, **M** ≤1wk, **L** ≤2wk, **XL** >2wk

---

## Epic REG-DOC — Documentation (Phase 0)

| ID | Task | Est | Dep | Status |
|---|---|---|---|---|
| REG-DOC-01 | Master plan corpus | M | — | ✅ |
| REG-DOC-02 | OpenAPI regenesis-v2.yaml draft | M | REG-DOC-01 | ❌ |
| REG-DOC-03 | Platform team review sign-off | S | REG-DOC-01 | ❌ |
| REG-DOC-04 | DCops team BootProfile ack | S | REG-DOC-01 | ❌ |

---

## Epic REG-AGENT — regenesis-agent (Phase 1)

| ID | Task | Est | Dep |
|---|---|---|---|
| REG-AGENT-01 | Shell script phases 1–10 per spec 06 | M | — |
| REG-AGENT-02 | `/etc/regenesis/config.yaml` template | S | 01 |
| REG-AGENT-03 | systemd units regenesis-agent + docs | S | 01 |
| REG-AGENT-04 | Integrate into multipass cloud-init Option B | M | 01–03 |
| REG-AGENT-05 | Parity checklist automation (test harness) | M | 04 |
| REG-AGENT-06 | Self-register replaces manual deploy-host-daemon | M | 04 |
| REG-AGENT-07 | Rust CLI port with phase markers | L | 01 |
| REG-AGENT-08 | Unit tests preflight/checksum/idempotent | M | 07 |
| REG-AGENT-09 | Publish binary to GitHub Releases | S | 07 |
| REG-AGENT-10 | ms02 E2E 3-node Multipass sign-off | S | 04–06 |

---

## Epic REG-PXE — iPXE artifacts (Phase 2)

| ID | Task | Est | Dep |
|---|---|---|---|
| REG-PXE-01 | Build ubuntu-24.04 netboot artifact pipeline | L | — |
| REG-PXE-02 | autoinstall user-data + late-commands | M | 01 |
| REG-PXE-03 | regenesis-config.yaml template per BootIntent | S | 02 |
| REG-PXE-04 | Finalize ipxe/cylon-resurrection.ipxe + localboot | S | — |
| REG-PXE-05 | HTTP artifact layout docs → PVC sync script | M | 01 |
| REG-PXE-06 | Lab E2E single bare-metal server | M | DCOPS-01 |

---

## Epic REG-DCOPS — DCops integration (Phase 2)

| ID | Task | Est | Dep | Owner |
|---|---|---|---|---|
| REG-DCOPS-01 | Implement pxe-server HttpServer (axum) | L | — | DCops |
| REG-DCOPS-02 | BootIntent → iPXE script rendering | M | 01 | DCops |
| REG-DCOPS-03 | lifecycle locked → localboot only | S | 02 | DCops |
| REG-DCOPS-04 | Example manifests namespace cylon-regenesis | S | — | regenesis |
| REG-DCOPS-05 | Kea DHCP options for ms02 lab VLAN | M | 01 | DCops |
| REG-DCOPS-06 | nginx lab workaround + runbook | S | — | regenesis |
| REG-DCOPS-07 | regenesis lifecycle callback API design | M | 02 | joint |

---

## Epic REG-CP — Control plane extraction (Phase 3)

| ID | Task | Est | Dep |
|---|---|---|---|
| REG-CP-3.1 | Bootstrap cylon-regenesis Cargo workspace | S | — |
| REG-CP-3.2 | Move resurrection-hub → regenesis-hub crate | M | 3.1 |
| REG-CP-3.3 | Typed RegenesisCommand Raft refactor | XL | 3.2 |
| REG-CP-3.4 | Tilt + K8s manifest update | M | 3.2 |
| REG-CP-3.5 | Remove crate from tiffany workspace | S | 3.4 |
| REG-CP-3.6 | CI github actions nextest clippy | M | 3.2 |
| REG-CP-3.7 | GET /v2/nodes + hibernate/resurrect HTTP | M | 3.2 |
| REG-CP-3.8 | HTTP mTLS middleware | M | 3.2 |

---

## Epic REG-CP-4 — Fault tolerance hardening (Phase 4)

| ID | Task | Est | Dep |
|---|---|---|---|
| REG-CP-4.1 | Hub offline reschedule integration tests | M | 3.2 |
| REG-CP-4.2 | CryoSleep S3 GC background leader task | L | 3.2 |
| REG-CP-4.3 | rejoin → DeleteCylonVm not UDS rm | M | tiffany |
| REG-CP-4.4 | SnapshotMetadata in Raft + S3 paths | L | 4.2 |
| REG-CP-4.5 | Chaos suite iptables/kill pod | L | 4.1 |
| REG-CP-4.6 | Host bid RPC or capacity refresh | M | 3.2 |
| REG-CP-4.7 | Complete MigrateCylonVm pipe | XL | tiffany |

---

## Epic REG-HOST — Cylon host gaps (tiffany)

| ID | Task | Est | Dep |
|---|---|---|---|
| REG-HOST-3.1 | OCI RegistryAuth from GITHUB_TOKEN | M | — |
| REG-HOST-3.2 | /metrics Prometheus endpoint | S | — |
| REG-HOST-3.3 | DeleteCylonVm on rejoin kill list | M | — |
| REG-HOST-4.1 | MigrateCylonVm memory streaming | XL | — |

---

## Epic REG-PROD — Production (Phase 5)

| ID | Task | Est | Dep |
|---|---|---|---|
| REG-PROD-01 | 3-node hub HA manifest | L | 3.6 |
| REG-PROD-02 | Fleet BootIntent GitOps repo layout | M | DCOPS-04 |
| REG-PROD-03 | HTTPS PXE or TLS proxy | M | DCOPS-01 |
| REG-PROD-04 | On-call runbooks validated | M | 4.5 |
| REG-PROD-05 | N≥3 production nodes locked | L | 2+4 |

---

## Critical path (Gantt summary)

```
REG-AGENT-01..06 ──► REG-PXE-01..06 ──► REG-PROD-05
        │                                      ▲
REG-CP-3.1..2 ──► REG-CP-4.* ─────────────────┘
        │
REG-DCOPS-01 (parallel blocker for PXE E2E)
```

---

## Definition of Done (global)

- [ ] Code + tests merged
- [ ] Plan doc updated if behavior changed
- [ ] llmwiki log entry (tiffany or regenesis)
- [ ] Runbook step validated on ms02 where applicable
