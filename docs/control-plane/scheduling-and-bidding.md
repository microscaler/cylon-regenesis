# Scheduling and bidding

**Flintlock proposal 02, 08**

## Algorithm

1. Client `POST /v2/agents` with `{ vcpu, memory_mb, rootfs_ref, ... }`.
2. Leader enqueues request into **batch window** (25–50ms MPSC).
3. Leader sends **BidRequest** to all `Online` nodes (gRPC or hub↔host extension).
4. Each node responds **BidResponse**:

   ```
   utilization_score = f(available_vcpu, available_memory_mb, running_vm_count)
   ```

   Lower score wins.

5. Leader selects winner → Raft `CreateAgent` commit.
6. Leader calls winner `CreateCylonVm` with OCI ref.

## Fairness

Homogeneous nodes: running count tie-breaker. Heterogeneous (future): normalize by total capacity.

## Batch scheduling (proposal 08)

| Without batching | With batching |
|---|---|
| N requests → N × fleet RPC | N requests → 1 bid round |
| Leader CPU spike | Amortized scatter-gather |

Target: 5000 req/s ([RESURRECTION-HUB-PRD](../../cylon/docs/RESURRECTION-HUB-PRD.md) 3.1).

## Node-side bid calculation (cylon host extension)

Future RPC on cylon host or computed locally from hub cache:

```
score = (used_vcpu / total_vcpu) * 0.5 + (used_mem / total_mem) * 0.5
```

Reject bid if request exceeds available.

## Failure during schedule

| Event | Action |
|---|---|
| Winner dies before CreateCylonVm | Re-bid |
| CreateCylonVm fails | Mark agent Failed; optional retry |
| Raft commit fails | Return 503 to client |

## References

- `cylon/crates/resurrection-hub/src/allocator.rs`
- Flintlock doc 02 sequence diagram
