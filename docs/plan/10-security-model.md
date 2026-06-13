# 10 — Security model

## Trust zones

```
┌─────────────────────────────────────────┐
│ Zone A: Platform (Kind)                  │
│  portal, cylon-daemon, postgres           │
│  Trust: operator SSO, internal           │
└─────────────────┬───────────────────────┘
                  │ mTLS hub-client
┌─────────────────▼───────────────────────┐
│ Zone B: regenesis-hub (Raft)             │
│  agent registry, scheduling              │
└─────────────────┬───────────────────────┘
                  │ mTLS
┌─────────────────▼───────────────────────┐
│ Zone C: resurrection nodes               │
│  cylon host, Firecracker, guest agents   │
└─────────────────┬───────────────────────┘
                  │ vsock proxy + billing
┌─────────────────▼───────────────────────┐
│ Zone D: external SaaS / LLM / git         │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│ Zone M: DCops mgmt (PXE, IPAM)           │
│  separate from agent execution           │
└─────────────────────────────────────────┘
```

## mTLS certificate roles

| Cert | Holder | Purpose |
|---|---|---|
| `ca.crt` | all | Root of trust |
| `hub-client.crt` | platform + hub outbound | Hub → node gRPC |
| `server.crt` | each node | node gRPC server |
| Raft peer certs | hub replicas | inter-hub |

Dev source: `cylon/deployment-configuration/profiles/dev/.certs`

Production: internal CA rotation runbook (TBD).

## Token handling

| Secret | Storage | Never |
|---|---|---|
| GITHUB_TOKEN | `/etc/cylon/github-token` 0600 | in Git, user-data |
| workspace github_token | CryoSleep snapshot encrypt | plain in Raft JSON |
| ghcr pull | env or token file | cloud-init public |

## HTTP API auth (target REG-CP-3.8)

- Require client cert for all `/v2/*` except `/v2/health`
- RBAC roles: `admin`, `platform`, `node` (register only)

## iPXE attack surface

| Threat | Mitigation |
|---|---|
| Rogue PXE on VLAN | 802.1x / isolated provisioning VLAN |
| MITM HTTP boot | TLS Phase 5 |
| Reinstall locked node | BootIntent lifecycle gate |
| Stolen MAC | NetBox inventory + BootIntent approval |

## Detached mode security

Prevents split-brain agent writes to Zone D when hub quorum lost.

## Audit

| Event | Log |
|---|---|
| agent create | Raft log + OTEL trace |
| node register | journald + hub log |
| BootIntent change | Git commit (DCops) |
| regenesis-agent | structured journal |

## Compliance notes

- Agent workspace tokens encrypted at rest in S3 (target)
- Customer data in guest VM — not on hub disk beyond Raft metadata
