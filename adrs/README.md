# Architecture Decision Records

ADRs record **locked decisions** for cylon-regenesis. Status values: Proposed | Accepted | Superseded.

| ADR | Title | Status |
|---|---|---|
| [ADR-0001](ADR-0001-scope-and-non-goals.md) | Scope & non-goals | Superseded by ADR-0007 |
| [ADR-0002](ADR-0002-flintlock-replacement-not-fork.md) | Flintlock replacement, not fork | Superseded by ADR-0007 |
| [ADR-0003](ADR-0003-ipxe-via-dcops-not-tinkerbell.md) | Standard iPXE via DCops | Accepted |
| [ADR-0004](ADR-0004-repo-boundaries.md) | Repo boundaries (cylon / images / DCops) | Superseded by ADR-0007 |
| [ADR-0005](ADR-0005-dev-multipass-to-bare-metal.md) | Multipass dev → bare-metal prod path | Superseded by ADR-0007 |
| [ADR-0006](ADR-0006-planning-baseline-code-audit.md) | Planning baseline — code audit snapshot | Superseded by ADR-0007 |
| [ADR-0007](ADR-0007-deprecate-in-favour-of-aether-native-kvm.md) | Deprecate in favour of Aether native KVM | Accepted |

## Process

1. Propose ADR in a PR with Status: **Proposed**.
2. Review against [PRD](../docs/PRD.md) and sibling ADRs (especially DCops ADR-001).
3. Merge with Status: **Accepted**.
4. To change direction, write a new ADR that supersedes the old one.
