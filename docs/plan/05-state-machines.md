# 05 — State machines

## Agent lifecycle (hub)

```mermaid
stateDiagram-v2
    [*] --> Pending: POST /v2/agents
    Pending --> Running: CreateCylonVm OK
    Pending --> Failed: gRPC error / timeout
    Running --> CryoSleep: Hibernate / idle policy
    Running --> Resurrecting: Node offline reschedule
    CryoSleep --> Resurrecting: POST resurrect / failover
    Resurrecting --> Running: ResurrectCylonVm OK
    Resurrecting --> Failed: restore error
    Running --> Failed: unrecoverable host error
    Failed --> [*]: DELETE / tombstone
    CryoSleep --> [*]: GC TTL expired
```

### Transition table

| From | Event | To | Actor |
|---|---|---|---|
| — | create accepted | Pending | Hub |
| Pending | CreateCylonVm OK | Running | Hub |
| Pending | CreateCylonVm fail | Failed | Hub |
| Running | HibernateCylonVm | CryoSleep | Hub |
| Running | node Offline | Resurrecting | Hub leader |
| CryoSleep | ResurrectCylonVm | Resurrecting | Hub |
| Resurrecting | restore OK | Running | Hub |
| CryoSleep | TTL 7d | tombstone | Hub GC loop |
| Running | drain migrate | Running (new node) | Hub |

---

## CylonNode lifecycle (hub)

```mermaid
stateDiagram-v2
    [*] --> Online: POST /v2/register
    Online --> Online: heartbeat refresh
    Online --> Offline: 3× missed heartbeats (15s)
    Online --> Draining: POST drain
    Draining --> Offline: migrations done / manual
    Offline --> Online: register after recovery
    Offline --> [*]: decommission (manual tombstone)
```

### Heartbeat timing

| Parameter | Value | Source |
|---|---|---|
| Host ping interval | 5s | cylon main.rs |
| Hub offline threshold | 15s (`last_heartbeat + 15`) | hub main.rs |
| Host detach threshold | 30s (6 failed pings) | cylon main.rs |

**Note:** Host uses `POST /v2/register` as heartbeat; hub updates `last_heartbeat` each success.

---

## Host connection lifecycle (cylon watchdog)

```mermaid
stateDiagram-v2
    [*] --> Connected
    Connected --> Connected: register OK
    Connected --> Detached: 6 failed pings
    Detached --> Reconciling: register OK after detach
    Reconciling --> Connected: rejoin processed
    Detached --> Detached: still failing
```

### Detached Mode actions

1. Enumerate `/tmp/firecracker-*.sock`
2. `pause_instance()` each VM (async spawn)
3. Disable egress proxy (target — partial today)
4. Set `in_detached_mode = true`

### Rejoin actions

1. Collect active vm_ids from UDS filenames
2. `POST /v2/nodes/rejoin`
3. For each kill_vm_id: DeleteCylonVm (**target**) / UDS rm (today)
4. Clear detached flag

---

## BootIntent lifecycle (DCops)

```mermaid
stateDiagram-v2
    [*] --> discovered: CRD created
    discovered --> installing: PXE boot started
    installing --> installed: regenesis-agent success
    installed --> locked: operator GitOps / API
    locked --> discovered: RMA reprovision (manual)
    installing --> discovered: install fail + reset
```

### iPXE gate rules

| lifecycle | pxe-server behavior |
|---|---|
| `discovered` | Serve full install script |
| `installing` | Serve install (idempotent autoinstall) |
| `installed` | Chain `sanboot --no-describe --drive 0x80` OR local boot only |
| `locked` | HTTP 403 on install artifacts OR local boot only |

---

## Allocator batch cycle

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Collecting: first request received
    Collecting --> Collecting: more requests within 50ms
    Collecting --> Processing: timeout or 500 cap
    Processing --> Idle: all replies sent
```

Per-request outcome: `Ok(CylonNode)` | `Err("No Cylon nodes available...")`

---

## Drain migration (per agent)

```mermaid
stateDiagram-v2
    [*] --> OnDrainingNode
    OnDrainingNode --> Migrating: target found
    OnDrainingNode --> Failed: no target
    Migrating --> OnTargetNode: MigrateCylonVm OK
    Migrating --> Failed: gRPC error
```

Hub updates `allocated_node_id` in Raft on success.
