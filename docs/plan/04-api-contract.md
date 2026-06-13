# 04 — API contract

**Base URL:** `{HUB_API}/v2`  
**Content-Type:** `application/json`  
**Auth (target):** mTLS client cert `hub-client` — **not enforced today**

---

## POST /v2/register

Register or refresh a resurrection node. Called every **5s** by cylon host watchdog (acts as heartbeat).

### Request

```json
{
  "node_id": "resurrection-node-1",
  "grpc_endpoint": "https://127.0.0.1:50052",
  "available_memory_mb": 8192,
  "available_vcpu": 8
}
```

### Response

| Code | Body |
|---|---|
| 200 | `"Registered"` (plain text today) |
| 500 | Raft error string |

### Side effects

- Sets `last_heartbeat = now()`
- Sets `status = Online`
- Raft `client_write` upsert `node:{node_id}`

---

## POST /v2/nodes/rejoin

After partition recovery. Cylon host sends local VM manifest.

### Request

```json
{
  "node_id": "resurrection-node-1",
  "active_vm_ids": ["agent-abc", "agent-def"]
}
```

### Response

```json
{
  "kill_vm_ids": ["agent-def"],
  "keep_vm_ids": ["agent-abc"]
}
```

### Logic (current)

For each `vm_id` in `active_vm_ids`:

- If hub has agent with same `vm_id`, `allocated_node_id == node_id`, `status == Running` → **keep**
- Else → **kill**

### Host obligations after response

For each `kill_vm_ids`: `DeleteCylonVm` + cleanup (today: UDS delete only — **gap**)

---

## POST /v2/agents

Create agent microVM.

### Request

```json
{
  "id": "agent-550e8400-e29b-41d4-a716-446655440000",
  "vcpu": 2,
  "memory_in_mb": 2048,
  "root_volume_source": "ghcr.io/microscaler/cylon-rootfs-ubuntu:latest",
  "workspace_project": "hauliage",
  "workspace_git_url": "https://github.com/microscaler/hauliage.git",
  "workspace_branch": "main",
  "workspace_github_token": "<optional>"
}
```

### Response 201

```json
{
  "vm_id": "agent-550e8400-...",
  "allocated_node_id": "resurrection-node-2",
  "status": "Running",
  "status_updated_at": 1718280000,
  "spec": { "...": "..." }
}
```

### Errors

| Code | Condition |
|---|---|
| 500 | No node capacity, gRPC failure, Raft failure |

### Internal flow

1. Enqueue `AllocationRequest`
2. Batch allocator picks node (max mem fit)
3. gRPC `CreateCylonVm`
4. Raft `set_agent(Running)`
5. Optimistic capacity decrement

---

## GET /v2/agents/{vm_id}

Proxy status from authoritative host.

### Response 200

```json
{
  "state_code": 2,
  "message": "Running"
}
```

`state_code` maps to `CylonVmStatus.MacroState` enum numeric.

### Errors

| Code | Condition |
|---|---|
| 404 | Agent not in registry |
| 500 | Node offline, gRPC error |

---

## POST /v2/nodes/{node_id}/drain

Initiate drain + migration attempts.

### Response 200

```
Drain initiated. 3 migrated, 0 failed.
```

### Side effects

- Node → `Draining`
- For each Running agent on node: `MigrateCylonVm` to best Online target

---

## Planned endpoints (not implemented)

| Method | Path | Purpose |
|---|---|---|
| GET | `/v2/nodes` | Fleet listing |
| GET | `/v2/nodes/{id}` | Node detail |
| POST | `/v2/agents/{id}/hibernate` | CryoSleep |
| POST | `/v2/agents/{id}/resurrect` | Restore from S3 |
| DELETE | `/v2/agents/{id}` | Tear down |
| GET | `/v2/health` | Hub health |
| GET | `/v2/ready` | Raft leader ready |

OpenAPI target: `openapi/regenesis-v2.yaml` (Phase 0b).

---

## gRPC — CylonService (host)

Source: `cylon/crates/cylon/proto/cylon.proto`

| RPC | Caller | Purpose |
|---|---|---|
| `CreateCylonVm` | Hub | Boot agent |
| `GetCylonVm` | Hub proxy | Status |
| `DeleteCylonVm` | Hub / rejoin | Teardown |
| `HibernateCylonVm` | Hub / detach | Snapshot + pause |
| `ResurrectCylonVm` | Hub | Restore |
| `MigrateCylonVm` | Hub drain | Start migration |
| `ReceiveMigration` | Target host | Stream chunks |

### mTLS

- Server: node cert SAN matches `cylon-node`
- Client: hub uses `hub-client.crt` + CA

### Trace propagation

W3C `traceparent` in gRPC metadata (implemented).

---

## regenesis-agent → Hub (Phase 1)

Same as `POST /v2/register` after first-boot completes, with optional extended payload:

```json
{
  "node_id": "ms02-resurrection-1",
  "grpc_endpoint": "https://10.0.0.50:50052",
  "available_memory_mb": 65536,
  "available_vcpu": 32,
  "labels": {
    "site": "ms02",
    "provisioning": "multipass|ipxe",
    "regenesis_agent_version": "0.1.0"
  }
}
```

Hub ignores unknown fields.

---

## Error catalog (HTTP)

| Code | Error string pattern | Client action |
|---|---|---|
| 500 | `No Cylon nodes available` | Retry backoff |
| 500 | `Failed to dial node` | Check mTLS + endpoint |
| 500 | `Failed to create VM` | Check OCI/GHCR on node |
| 500 | `Raft Commit Failed` | Retry; hub may be follower |
| 404 | `Agent not found` | Refresh registry |

See [13-work-breakdown-structure.md](13-work-breakdown-structure.md) for structured error types (future).
