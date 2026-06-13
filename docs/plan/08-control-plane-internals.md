# 08 — Control plane internals

Maps **regenesis-hub** (current `resurrection-hub`) to files and runtime loops.

## Process architecture

```
main.rs
├── OpenRaft init (node_id from HUB_NODE_ID)
├── HubState::new(alloc_tx)
├── allocator::start_allocation_loop(alloc_rx)
├── heartbeat monitor task (leader only)
├── optional CYLON_GRPC_ENDPOINT ping task
└── axum serve :14000 api::router
```

## HubState fields

| Field | Type | Purpose |
|---|---|---|
| `id` | MemNodeId | Raft peer id |
| `raft` | HubRaft | Consensus |
| `store` | Arc<MemStore> | Snapshot + log |
| `tls_config` | Option<ClientTlsConfig> | mTLS to nodes |
| `routing_table` | RwLock<HashMap<NodeId, String>> | Raft peer addrs |
| `alloc_tx` | mpsc::Sender<AllocationRequest> | Scheduler queue |
| `serial` | AtomicU64 | Client write serial |

## Raft configuration (current)

```rust
heartbeat_interval: 500ms
election_timeout_min: 1500ms
election_timeout_max: 3000ms
snapshot_policy: LogsSinceLast(10_000)
```

## HTTP routes

| Route | Handler | Leader required? |
|---|---|---|
| POST /v2/agents | create_agent | de facto leader for writes |
| GET /v2/agents/{id} | get_agent_status | any |
| POST /v2/register | register_node | any (Raft write) |
| POST /v2/nodes/rejoin | rejoin_node | any |
| POST /v2/nodes/{id}/drain | drain_node | leader recommended |
| POST /raft/* | raft ops | admin |

## create_agent critical path (latency)

| Step | Typical ms | Failure |
|---|---|---|
| JSON deserialize | <1 | 400 |
| alloc_tx send | <1 | 500 queue full |
| batch wait | 0–50 | — |
| node selection | <1 | 500 no capacity |
| gRPC connect | 10–100 | 500 dial |
| CreateCylonVm | 5000–120000 | 500 OCI pull |
| Raft set_agent | 10–50 | 500 commit |

**P99 target:** <120s dominated by OCI first pull; scheduling <100ms.

## Heartbeat monitor (main.rs)

Every 5s, if leader:

```rust
for node in get_nodes() {
  if Online && now > last_heartbeat + 15 {
    mark Offline, zero capacity
    for agent on node {
      reschedule // Resurrect or Create from snapshot
    }
  }
}
```

Read full loop in `resurrection-hub/src/main.rs` lines 142+.

## Allocator algorithm detail

Current: **best-fit by max available_memory_mb** among Online nodes satisfying vcpu+mem.

Not yet implemented from Flintlock doc 02:

- Explicit bid RPC to each host
- Utilization score formula on host-reported metrics

**Target Phase 4:**

```
score = w1 * (used_vcpu/total_vcpu) + w2 * (used_mem/total_mem) + w3 * running_vm_count
```

## Raft state machine (current — JSON hack)

`ClientRequest { client: "node:foo" | "agent:bar", serial, status: json|string }`

Apply:

- `status == "TOMBSTONE"` → delete key
- else → upsert JSON

**Refactor epic REG-CP-3.2:** replace with `RegenesisCommand` bincode/protobuf.

## Snapshot format

OpenRaft install_snapshot via memstore — contains full `client_status` map.

Restore procedure (runbook):

1. Stop hub pod
2. Clear data dir if corrupted
3. Bootstrap from snapshot object
4. `raft/init` if new cluster

## Leader-only background jobs (target)

| Job | Interval | Phase |
|---|---|---|
| Heartbeat monitor | 5s | ✅ |
| CryoSleep GC | 1h | ❌ |
| Snapshot upload S3 | on snapshot | ❌ |
| Metrics scrape self | 15s | partial |

## Migration checklist (Phase 3)

1. Copy `crates/resurrection-hub` → `cylon-regenesis/crates/regenesis-hub`
2. Rename package in Cargo.toml
3. Update Tiltfile image build context
4. Cylon depends on git/path crate; remove local crate
5. Run full integration test suite
6. Update import paths in docs

## File map

| File | LOC responsibility |
|---|---|
| `api.rs` | HTTP handlers, gRPC client calls |
| `allocator.rs` | Batch scheduler |
| `state.rs` | DTOs, HubState helpers |
| `raft/mod.rs` | TypeConfig, ClientRequest apply |
| `raft/network.rs` | HubNetwork gRPC between peers |
| `telemetry.rs` | OTEL init |
| `main.rs` | Wiring, background tasks |

## Performance limits (design)

| Resource | Limit |
|---|---|
| alloc channel | 10000 |
| batch size | 500 |
| batch window | 50ms |
| max agents in memory | 1M (theoretical; snapshot size risk) |
