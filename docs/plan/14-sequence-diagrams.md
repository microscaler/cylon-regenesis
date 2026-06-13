# 14 — Sequence diagrams

## SD-01 Agent create (happy path)

```mermaid
sequenceDiagram
    participant P as Platform
    participant H as regenesis-hub
    participant R as Raft
    participant N as cylon host
    participant FC as Firecracker
    participant GHCR as GHCR

    P->>H: POST /v2/agents
    H->>H: enqueue allocation batch
    H->>H: pick node (max mem)
    H->>N: gRPC CreateCylonVm
    N->>GHCR: OCI pull rootfs
    N->>FC: start microVM
    N-->>H: OK
    H->>R: commit agent Running
    H-->>P: 201 AgentInstance
```

## SD-02 Node registration heartbeat

```mermaid
sequenceDiagram
    participant N as cylon host
    participant H as regenesis-hub
    participant R as Raft

    loop every 5s
        N->>H: POST /v2/register
        H->>R: upsert node Online
        H-->>N: 200
    end
```

## SD-03 Detached mode

```mermaid
sequenceDiagram
    participant N as cylon host
    participant H as regenesis-hub
    participant FC as Firecracker

    loop failed pings
        N-xH: register fails
    end
    Note over N: 6 failures = 30s
    N->>FC: pause all VMs
    Note over N: Detached Mode
    N->>H: register OK
    N->>H: POST /v2/nodes/rejoin
    H-->>N: kill_vm_ids / keep_vm_ids
    N->>FC: delete killed VMs
```

## SD-04 Hub marks node offline

```mermaid
sequenceDiagram
    participant H as hub leader
    participant R as Raft
    participant N2 as cylon host-2

    Note over H: no heartbeat 15s
    H->>R: node Offline cap=0
    H->>H: list agents on node
    H->>N2: ResurrectCylonVm or Create
    H->>R: update agent placement
```

## SD-05 iPXE bare metal regenesis

```mermaid
sequenceDiagram
    participant BM as bare metal
    participant DHCP as Kea
    participant PXE as DCops pxe-server
    participant OS as Ubuntu autoinstall
    participant RA as regenesis-agent
    participant H as regenesis-hub

    BM->>DHCP: DISCOVER
    DHCP-->>BM: OFFER + ipxe
    BM->>PXE: GET cylon-resurrection.ipxe
    BM->>PXE: GET kernel/initrd
    BM->>OS: boot installer
    OS->>RA: late-command install
    RA->>RA: phases preflight→finalize
    RA->>H: POST /v2/register
    RA->>RA: touch configured
```

## SD-06 BootIntent lock

```mermaid
sequenceDiagram
    participant Op as Operator Git
    participant DC as DCops controller
    participant PXE as pxe-server
    participant BM as server

    Op->>DC: BootIntent lifecycle locked
    BM->>PXE: PXE request
    PXE-->>BM: localboot.ipxe only
    Note over BM: no reinstall
```

## SD-07 Drain migration (target)

```mermaid
sequenceDiagram
    participant Op as Admin
    participant H as hub
    participant Nd as draining node
    participant Nt as target node

    Op->>H: POST drain node-d
    H->>Nd: MigrateCylonVm(agent, target)
    Nd->>Nt: ReceiveMigration stream
    Nt->>Nt: resume VM
    H->>H: Raft update node_id
```
