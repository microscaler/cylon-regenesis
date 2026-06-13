# Flintlock distributed architecture — requirements baseline

## Origin

Microscaler submitted this enhancement package to the **Liquid Metal governance board** (June 2025). It describes gaps in upstream Flintlock and proposed remedies using Raft, distributed scheduling, PXE regenesis, and operational hardening.

**We are not implementing this in upstream Flintlock.** cylon-regenesis implements the same capabilities for CRP. These documents are the **requirements baseline** — traceability only.

## Source location (ms02)

```
~/Workspace/liquidmetal/flintlock/docs/proposals/distributed_architecture/
├── README.md
├── coverletter.md
└── docs/
    ├── 01-Raft_Consensus_Integration.md
    ├── 02-Distributed_Scheduling_and_Bidding_Mechanism.md
    … (through 15)
```

Do not edit copies in this repo unless adding CRP-specific annotations. Prefer [flintlock-requirements-mapping.md](flintlock-requirements-mapping.md) for implementation status.

## Strategic shift

| June 2025 proposal | 2026 CRP decision |
|---|---|
| Enhance Flintlock in Liquid Metal | Build **cylon-regenesis** (ADR-0002) |
| Flatcar + PXE Flintlock host | Ubuntu host + **Cylon host daemon** |
| Flintlock gRPC API | **`cylon.proto`** + Hub `/v2` |
| Tinkerbell (community context) | **DCops + standard iPXE** (ADR-0003) |

## Cover letter note

The original cover letter explicitly avoided forking to preserve Flintlock's user base. CRP is a **different product surface** (AI agent microVMs) — forking Flintlock would not serve CRP users. We retain the *technical* proposals, not the governance submission strategy.

See [ADR-0002](../adrs/ADR-0002-flintlock-replacement-not-fork.md).
