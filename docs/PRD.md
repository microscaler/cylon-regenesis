# Product Requirements — Cylon Regenesis

**Owner:** Platform / FAR  
**Status:** Active — Phase 0 (documentation)  
**Related:** [ARCHITECTURE.md](ARCHITECTURE.md), [cylon CRP PRD](../../cylon/docs/CYLON-RESURRECTION-PLATFORM-PRD.md), [cylon RESURRECTION-HUB-PRD](../../cylon/docs/RESURRECTION-HUB-PRD.md)

## 1. Vision

Deliver a **Microscaler-owned distributed microVM platform** that replaces Flintlock/Liquidmetal for CRP: schedule agent Firecracker VMs, resurrect them after failure, and **regenerate bare-metal hosts** via standard iPXE orchestrated by DCops.

## 2. Personas

| Persona | Need |
|---|---|
| **Platform engineer** | Hub API, node registration, scheduling fairness |
| **DC operator** | iPXE profiles, IPAM, safe reinstall lock |
| **Agent developer** | Stable `CreateCylonVm` + GHCR rootfs — no host provisioning detail |

## 3. Functional requirements

### FR-1 Host regenesis (iPXE)

| ID | Requirement | Acceptance |
|---|---|---|
| FR-1.1 | Declarative boot via DCops `BootIntent` + `BootProfile` | MAC → profile reconciled; iPXE script served over HTTP |
| FR-1.2 | Standard iPXE chain (no Tinkerbell) | `dhcp` → `chain http://.../cylon-resurrection.ipxe` → boot |
| FR-1.3 | First-boot `regenesis-agent` idempotent | Second run is no-op when `Installed` |
| FR-1.4 | Parity with Multipass cloud-init | [cloud-init-parity.md](host-regenesis/cloud-init-parity.md) checklist 100% |
| FR-1.5 | Secure Hub registration post-boot | mTLS cert issued; `POST /v2/register` succeeds |
| FR-1.6 | BootIntent → `locked` after success | Prevents accidental reinstall |

### FR-2 Control plane

| ID | Requirement | Acceptance |
|---|---|---|
| FR-2.1 | OpenRaft replicated agent/node state | 3-node hub tolerates 1 failure |
| FR-2.2 | Bidding scheduler | Load spread ±10% across homogeneous nodes |
| FR-2.3 | API v2 + authoritative proxy | `GET /v2/agents/{id}/status` matches host truth |
| FR-2.4 | Node heartbeat + offline detection | 15s failover reschedule (RESURRECTION-HUB-PRD 2.1) |
| FR-2.5 | Detached host GC | Host stops VMs within 30s of hub loss (PRD 2.2) |
| FR-2.6 | Rejoin reconciliation | No duplicate agents after partition (PRD 2.3) |
| FR-2.7 | S3 snapshot GC | CryoSleep >7d purged (PRD 4.1) |

### FR-3 Integration

| ID | Requirement | Acceptance |
|---|---|---|
| FR-3.1 | `cylon.proto` source in cylon | regenesis-hub compiles against same revision |
| FR-3.2 | GHCR guest artifacts | Host pulls `cylon-kernel` + hub specifies rootfs ref |
| FR-3.3 | DCops IPClaim for node mgmt IP | No hard-coded resurrection-node IPs in Git |

## 4. Non-functional requirements

| ID | Requirement | Target |
|---|---|---|
| NFR-1 | Hub burst scheduling | 5k agents/s batch path (PRD 3.1) |
| NFR-2 | End-to-end trace | Hub → host → Firecracker span |
| NFR-3 | Host reprovision time | <15 min bare metal to `registered` (Phase 2) |
| NFR-4 | Documentation | Every phase has runbook + rollback |

## 5. Phase index

| Phase | Doc | Goal | Exit criteria |
|---|---|---|---|
| **0** | [phase-0](phases/phase-0-docs-and-contracts.md) | Docs, ADRs, contracts | This repo README + ARCHITECTURE approved |
| **1** | [phase-1](phases/phase-1-multipass-parity.md) | regenesis-agent on Multipass | Agent replaces cloud-init runcmd block |
| **2** | [phase-2](phases/phase-2-dcops-ipxe-dev.md) | Lab bare metal iPXE | 1 node boots via DCops only |
| **3** | [phase-3](phases/phase-3-control-plane-extraction.md) | Hub crate move | Cylon depends on regenesis-hub crate |
| **4** | [phase-4](phases/phase-4-fault-tolerance.md) | Production resilience | Chaos tests pass PRD 2.x |
| **5** | [phase-5](phases/phase-5-production-bare-metal.md) | DC rollout | N nodes locked in DCops |

## 6. Out of scope (this PRD)

- Flintlock API compatibility
- Tinkerbell workflows
- CAPMVM / Cluster API
- Guest rootfs Dockerfile changes (see cylon-images)
- Portal UX (see FAR-TILT-TUNNEL-PRD)

## 7. Success metrics

| Metric | Baseline | Target |
|---|---|---|
| Manual host rebuild time | ~2h (Multipass manual) | <15 min iPXE |
| Agent reschedule after node death | manual | <15s automatic |
| Hub/agent schema drift incidents | ad hoc | 0 per release (contract tests) |
