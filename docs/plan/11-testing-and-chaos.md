# 11 — Testing & chaos

## Test pyramid

```
        ┌─────────────┐
        │ Chaos / E2E │  ms02 bare metal, partition tests
        ├─────────────┤
        │ Integration │  hub + multipass node + Kind
        ├─────────────┤
        │ Unit        │  allocator, rejoin logic, agent phases
        └─────────────┘
```

## Unit tests (existing)

| Crate | File | Coverage |
|---|---|---|
| resurrection-hub | `allocator.rs` mod tests | batch fill, skip insufficient |
| resurrection-hub | integration.rs | partial |

**Target:** regenesis-agent phase tests in cylon-regenesis.

## Integration tests

### INT-01 Multipass fleet smoke

```
just build-base && clone-node 1..3
regenesis-agent configured on each
just resurrection-nodes-status
just resurrection-nodes-smoke-cylon-rootfs-ubuntu
```

### INT-02 Hub register heartbeat

Simulate 20s hub stop → node enters detached → restore → rejoin.

### INT-03 Create agent placement

POST 10 agents → verify spread across 3 nodes ±1.

## Chaos matrix

| ID | Scenario | Setup | Pass |
|---|---|---|---|
| CHAOS-01 | Node network partition | iptables DROP hub IP on node-1 | VMs paused <30s |
| CHAOS-02 | Node death | multipass stop node-1 | reschedule <15s |
| CHAOS-03 | Hub leader kill | delete resurrection-hub pod | new leader <5s |
| CHAOS-04 | Hub split brain | partition 2/1 raft | minority rejects write |
| CHAOS-05 | Rejoin duplicates | restart node with stale VMs | kill list honored |
| CHAOS-06 | OCI pull fail | bad image ref | agent Failed, no leak |
| CHAOS-07 | PXE reinstall locked | PXE while locked | local boot only |

## Performance tests

| ID | Load | Pass |
|---|---|---|
| PERF-01 | 100 sequential POST /v2/agents | all 201 |
| PERF-02 | 500 burst in 1s | batch allocator no timeout |
| PERF-03 | 1000 GET proxy status | p99 <500ms |

## CI pipeline (target)

```yaml
# cylon-regenesis/.github/workflows/ci.yml
- cargo fmt --check
- cargo clippy
- cargo nextest run
- optional: Kind deploy smoke job on main
```

## Test environments

| Env | Use |
|---|---|
| macOS + ms02 SSH | primary dev |
| Kind cylon + Multipass | integration |
| DCops Kind + lab metal | Phase 2+ |
| Production shadow | Phase 5 canary |

## Regression gates

No Phase promotion without:

- All unit tests green
- Relevant integration suite green
- Chaos subset for phase (see phase docs)
