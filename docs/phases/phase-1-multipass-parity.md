# Phase 1 — Multipass parity

**Depends on:** Phase 0  
**Goal:** Extract first-boot into **regenesis-agent**; Multipass nodes self-register with hub.

## Work items

### 1.1 regenesis-agent shell prototype

- `scripts/regenesis-agent` — phases per [06-regenesis-agent-spec.md](../plan/06-regenesis-agent-spec.md)
- Config: `/etc/regenesis/config.env` (shell-sourceable)

### 1.2 Multipass integration

- Minimal `cloud-init.yaml` (users + token + release-pin)
- `build-base` calls `just -f cylon-regenesis provision-base`

### 1.3 Hub registration

- `provision-node` / `just provision-fleet` — `POST /v2/register` + health

### 1.4 Tiffany integration

- `just resurrection-nodes-finish-regenesis` → `cylon-regenesis just provision-fleet`
- `resurrection-nodes-deploy-host-daemon` aliases finish-regenesis

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
