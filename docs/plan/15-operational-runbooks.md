# 15 — Operational runbooks

## RB-01 Replace failed resurrection node (production)

**When:** hardware RMA, unrecoverable OS corruption  
**Prereq:** agents on node hibernated or rescheduled

1. Hub confirms node `Offline` or admin drains node
2. Verify no `Running` agents on node in hub registry
3. Git: set BootIntent `lifecycle: discovered` for new MAC (if changed)
4. Update BootIntent MAC if hardware swap
5. Power on server → PXE boot
6. Wait autoinstall + regenesis-agent (<15 min)
7. Verify `curl hub` shows node `Online`
8. Git: BootIntent `lifecycle: locked`
9. Run smoke: `POST /v2/agents` single test agent

**Rollback:** power off; revert BootIntent to locked previous profile

---

## RB-02 Multipass dev node rebuild (Phase 1)

```bash
cd ~/Workspace/microscaler/cylon-images/multipass
just purge-base && just build-base
cd ~/Workspace/microscaler/cylon-local-infra
just resurrection-nodes-up
cd ~/Workspace/microscaler/cylon
just resurrection-nodes-status
```

If regenesis-agent enabled: skip `resurrection-nodes-deploy-host-daemon` except certs if needed.

---

## RB-03 Force node detached mode test

```bash
# on ms02 — block hub from node-1
multipass exec resurrection-node-1 -- sudo iptables -A OUTPUT -d 192.168.1.189 -j DROP
# wait 35s — expect paused VMs in journal
multipass exec resurrection-node-1 -- sudo iptables -D OUTPUT -d 192.168.1.189 -j DROP
# expect rejoin log lines
```

---

## RB-04 Hub leader failure

```bash
kubectl delete pod -l app=resurrection-hub -n tiffany  # K8s namespace unchanged during repo rename
# wait for new pod
curl -sf http://127.0.0.1:14000/v2/agents  # via tilt forward
```

Raft should elect new leader; verify logs.

---

## RB-05 GHCR guest kernel refresh on fleet

```bash
cd ~/Workspace/microscaler/cylon-images
just stage-kernel-from-ghcr-all
# or regenesis-agent --phase guest_kernel --force per node
```

Verify sha256 on each node.

---

## RB-06 Cylon host binary upgrade

```bash
cd ~/Workspace/microscaler/cylon
just resurrection-nodes-install-binary-from-release
# or set /etc/cylon/release-pin=v0.1.1 and restart regenesis-agent phase cylon_binary
```

---

## RB-07 PXE nginx lab workaround (Phase 2a)

1. Build artifacts to `~/Workspace/microscaler/cylon-regenesis/artifacts/pxe`
2. Deploy nginx manifest mounting hostPath
3. Point Kea next-server to nginx ClusterIP
4. Apply BootProfile/BootIntent
5. PXE boot lab server

Remove when DCops pxe-server HTTP live.

---

## RB-08 CryoSleep agent manual purge (future GC)

```
POST /v2/agents/{id}/hibernate  # if not already
# wait for S3 snapshot
DELETE /v2/agents/{id}          # when implemented
```

---

## Escalation

| Symptom | Check | Owner |
|---|---|---|
| All agents fail create | GHCR pull on nodes | platform |
| Node flapping Offline | network, register 5s | network |
| PXE loop | BootIntent lifecycle | DCops |
| Raft no leader | hub pod logs, peers | platform |
| Duplicate agents post-partition | rejoin kill list | cylon host |

---

## Log locations

| Component | Where |
|---|---|
| regenesis-hub | kubectl logs resurrection-hub |
| cylon host | multipass exec journalctl -u cylon-host |
| regenesis-agent | journalctl -u regenesis-agent |
| pxe-server | kubectl logs -n dcops-system |
