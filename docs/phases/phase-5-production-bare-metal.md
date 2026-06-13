# Phase 5 — Production bare metal

**Depends on:** Phase 2 (iPXE), Phase 4 (resilience)  
**Goal:** N resurrection nodes in production DC on DCops GitOps + HA regenesis-hub.

## Target topology

| Component | Count | Notes |
|---|---|---|
| regenesis-hub Raft peers | 3 | mgmt cluster or dedicated VMs |
| resurrection nodes | N | rack-mounted x86_64 + KVM |
| DCops pxe-server | 2+ | HA behind load balancer |
| Kea DHCP | 2 | failover pair |
| NetBox | existing | SoT for MAC/IP |

## Work items

### 5.1 HA hub cluster

- Move from Kind dev to production mgmt K8s
- Persistent volumes for Raft snapshots
- mTLS cert rotation runbook

### 5.2 Fleet BootIntent GitOps

- One BootIntent per physical MAC
- All `lifecycle: locked` after successful regenesis
- IPPool per site

### 5.3 Monitoring

- Grafana dashboards ([observability.md](../control-plane/observability.md))
- Alert: node offline >1m, regenesis boot fail, Raft no leader

### 5.4 Operational procedures

| Procedure | Doc |
|---|---|
| Replace failed server | Reset BootIntent → PXE → lock |
| Drain node for maintenance | `POST /v2/nodes/{id}/drain` |
| Emergency detach | Hub marks offline |

### 5.5 Decommission Multipass dev path

- ms02 lab keeps Multipass for dev only
- Production runbooks exclude Multipass

## Acceptance criteria

- [ ] Production N≥3 nodes registered
- [ ] Agent create/resurrect/hibernate under load
- [ ] Rack replacement <15 min via iPXE
- [ ] On-call runbook reviewed

## Success metrics (PRD)

| Metric | Target |
|---|---|
| Manual host rebuild | <15 min |
| Agent failover | <15s |
| Unplanned split-brain incidents | 0 |
