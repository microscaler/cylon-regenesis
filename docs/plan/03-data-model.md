# 03 — Data model

All hub-authoritative entities replicated via OpenRaft unless noted.

## Storage layout (current implementation)

Raft state machine: `openraft_memstore::MemStore` with key-value map `client_status`:

| Key prefix | Value JSON type | Tombstone |
|---|---|---|
| `node:{node_id}` | `CylonNode` | overwrite with Offline/zero cap |
| `agent:{vm_id}` | `AgentInstance` | `"TOMBSTONE"` string |

**Target (Phase 3 refactor):** typed `RaftCommand` enum — see [08-control-plane-internals.md](08-control-plane-internals.md).

---

## CylonNode

Physical or virtual hypervisor running the Cylon host daemon.

```rust
pub struct CylonNode {
    pub node_id: String,           // unique, e.g. "resurrection-node-1"
    pub grpc_endpoint: String,     // "https://192.168.1.x:50052"
    pub available_memory_mb: u64,  // schedulable remainder
    pub available_vcpu: u32,
    pub status: NodeStatus,
    pub last_heartbeat: u64,       // unix seconds
}
```

### NodeStatus

| Variant | Scheduling | Transitions |
|---|---|---|
| `Online` | Accepts bids | register, heartbeat refresh |
| `Offline` | Excluded; capacity 0 | hub monitor @15s missed heartbeats |
| `Draining` | No new bids; migrate existing | `POST /v2/nodes/{id}/drain` |

### Capacity accounting

- **Register payload** reports total available (from env or auto-detect future).
- **Optimistic decrement** on each `CreateCylonVm` success in hub.
- **Heartbeat** refresh should eventually report **actual** free capacity from host (gap: today static env).

### Indexes (logical)

- Primary: `node_id`
- Query: all nodes where `status == Online`

---

## AgentInstance

One Firecracker microVM = one AI agent.

```rust
pub struct AgentInstance {
    pub vm_id: String,
    pub allocated_node_id: String,
    pub status: AgentState,
    pub status_updated_at: Option<u64>,
    pub spec: CreateAgentDto,
}
```

### CreateAgentDto

```rust
pub struct CreateAgentDto {
    pub id: String,                      // vm_id
    pub vcpu: i32,
    pub memory_in_mb: i32,
    pub root_volume_source: Option<String>,  // OCI ref, default GHCR ubuntu
    pub workspace_project: Option<String>,
    pub workspace_git_url: Option<String>,
    pub workspace_branch: Option<String>,
    pub workspace_github_token: Option<String>,
}
```

**Defaults (hub should apply if omitted):**

| Field | Default |
|---|---|
| `root_volume_source` | `ghcr.io/microscaler/cylon-rootfs-ubuntu:latest` |
| `vcpu` | 2 |
| `memory_in_mb` | 2048 |

### AgentState (hub macro state)

| State | Meaning | Cylon MacroState mapping |
|---|---|---|
| `Pending` | Raft committed; gRPC in flight | PENDING |
| `Running` | CreateCylonVm OK | RUNNING |
| `CryoSleep` | Snapshot offloaded | CRYO_SLEEP |
| `Resurrecting` | Restore in progress | RESURRECTING |

---

## SnapshotMetadata (target — not in code)

```rust
pub struct SnapshotMetadata {
    pub vm_id: String,
    pub memory_uri: String,      // s3://bucket/agents/{vm_id}/memory.snap
    pub disk_uri: String,
    pub created_at: u64,
    pub last_accessed_at: u64,
}
```

Stored in Raft or sidecar object index. Required for offline node reschedule.

---

## RegenesisHostRecord (target — regenesis-agent / DCops)

Not in Raft — DCops + optional hub mirror.

```yaml
node_id: resurrection-rack1-u42
mac_address: "aa:bb:cc:dd:ee:ff"
management_ip: 192.168.1.50
boot_intent: ms02-lab-resurrection-01
host_os_version: ubuntu-24.04-regenesis-2026.06
cylon_release_pin: v0.1.1
guest_kernel_ref: ghcr.io/microscaler/cylon-kernel:6.1.102
registered_at: 2026-06-13T12:00:00Z
regenesis_agent_version: 0.1.0
```

---

## DCops BootIntent (external CRD)

```yaml
spec:
  macAddress: string
  profileRef: { name, namespace? }
  lifecycle: discovered | installing | installed | locked
status:
  configured: bool
  lifecycle: enum
  lastReconciled: timestamp
  error: string?
```

See [07-ipxe-dcops-spec.md](07-ipxe-dcops-spec.md).

---

## Identifier conventions

| Entity | Pattern | Example |
|---|---|---|
| node_id | `{site}-{role}-{n}` or BootIntent name | `ms02-resurrection-1` |
| vm_id | UUID or prefixed | `agent-{uuid}`, `cylon-ubuntu-smoke-{ts}` |
| BootIntent | `{site}-{rack}-{u}` | `ms02-lab-u42` |

---

## Consistency rules

1. **Single authoritative node** per `Running` agent in Raft.
2. **Rejoin:** local VM not in hub `Running` on same node → kill.
3. **Offline node:** hub sets capacity 0 before reschedule attempts.
4. **Tombstone** agent keys excluded from list queries.
5. **Draining node:** no new allocations; existing migrated or failed logged.

---

## Future: typed Raft log entry

```rust
enum RegenesisCommand {
    UpsertNode(CylonNode),
    RemoveNode { node_id },
    UpsertAgent(AgentInstance),
    TombstoneAgent { vm_id },
    UpsertSnapshot(SnapshotMetadata),
    TombstoneSnapshot { vm_id },
}
```

Migration path: dual-read JSON + typed during Phase 3 extraction.
