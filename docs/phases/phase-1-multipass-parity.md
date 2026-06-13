# Phase 1 — Multipass parity

**Depends on:** Phase 0  
**Goal:** Extract first-boot into **regenesis-agent**; Multipass nodes self-register with hub.

## Work items

### 1.1 regenesis-agent shell prototype

- Create `scripts/regenesis-agent.sh` implementing [first-boot-sequence.md](../host-regenesis/first-boot-sequence.md)
- Idempotency marker `/var/lib/regenesis/configured`

### 1.2 Multipass integration

- Option A: Replace `runcmd` tail in `cylon-images/multipass/cloud-init.yaml` with regenesis-agent install + invoke
- Option B: Keep cloud-init packages; only delegate steps 4–10 to agent

Prefer **Option B** initially — smaller cloud-init diff.

### 1.3 Hub registration

- Implement or wire `POST /v2/register` in resurrection-hub (tiffany) if missing
- regenesis-agent writes `/etc/cylon/host.env` from template + node id

### 1.4 Parity verification

- Run [cloud-init-parity.md](../host-regenesis/cloud-init-parity.md) checklist on all 3 nodes
- `just resurrection-nodes-status` green
- `just resurrection-nodes-smoke-cylon-rootfs-ubuntu` passes

### 1.5 Rust CLI (optional in Phase 1)

- `crates/regenesis-agent` — port shell script

## Acceptance criteria

- [ ] New Multipass base boots without manual `resurrection-nodes-deploy-host-daemon`
- [ ] All parity checklist items pass
- [ ] GHCR guest kernel on node (not S3 quickstart)
- [ ] Documented rollback: revert cloud-init + rebuild base

## Runbook

```bash
# ms02
cd ~/Workspace/microscaler/cylon-images/multipass
just purge-base && just build-base
cd ~/Workspace/microscaler/cylon-local-infra
just resurrection-nodes-up
cd ~/Workspace/microscaler/tiffany
just resurrection-nodes-status
```

## Risks

| Risk | Mitigation |
|---|---|
| Token sync still manual | regenesis-agent reads `/etc/cylon/github-token` injected at build-base |
| mTLS cert chicken-and-egg | Phase 1 keeps `resurrection-nodes-deploy-host-daemon` for certs only |

## Next phase

[Phase 2 — DCops iPXE lab](phase-2-dcops-ipxe-dev.md)
