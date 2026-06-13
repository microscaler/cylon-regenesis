# Phase 0 — Documentation & contracts

**Status:** In progress (2026-06-13)  
**Goal:** Establish canonical design before Rust crates land.

## Deliverables

| Item | Path | Status |
|---|---|---|
| Root README | `README.md` | Done |
| Agent rules | `AGENTS.md` | Done |
| Architecture | `docs/ARCHITECTURE.md` | Done |
| PRD | `docs/PRD.md` | Done |
| Repo map | `docs/REPO-MAP.md` | Done |
| ADRs 0001–0005 | `adrs/` | Done |
| Flintlock mapping | `docs/proposals/` | Done |
| Host regenesis specs | `docs/host-regenesis/` | Done |
| Control plane specs | `docs/control-plane/` | Done |
| Phase plans | `docs/phases/` | Done |

## Contract artifacts (next)

| Artifact | Purpose | Owner |
|---|---|---|
| `openapi/regenesis-v2.yaml` | Hub HTTP API | Phase 0b |
| `proto/` symlink or dep doc | cylon.proto revision pin | Phase 0b |
| `ipxe/cylon-resurrection.ipxe` | Canonical script (stub OK) | Phase 1 |
| Example BootProfile YAML | DCops GitOps | Phase 2 |

## Acceptance criteria

- [x] ADRs accepted for scope, iPXE/DCops, repo boundaries
- [x] ARCHITECTURE distinguishes host kernel vs guest vmlinux
- [x] Flintlock 15-doc mapping complete
- [ ] Platform team review sign-off
- [ ] DCops team acknowledges BootProfile use case (x86_64 CRP)

## Rollback

N/A — docs only.

## Next phase

[Phase 1 — Multipass parity](phase-1-multipass-parity.md)
