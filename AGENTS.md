# Cylon Regenesis — agent rules

## Before you do anything

1. Read [docs/README.md](docs/README.md) — documentation index.
2. Read [adrs/README.md](adrs/README.md) — locked decisions.
3. Read [docs/REPO-MAP.md](docs/REPO-MAP.md) — sibling repo boundaries.

## Scope

- **In scope:** host regenesis (iPXE via DCops), control-plane design, migration from `resurrection-hub`, integration contracts.
- **Out of scope:** agent runtime (`cylon/crates/engine`), guest rootfs builds (`cylon-images`), Talos/CAPI cluster lifecycle (`DCops` Phase 2+).

## Conventions

- ADRs live in `adrs/` — numbered `ADR-NNNN-short-title.md`. Do not edit accepted ADRs; supersede with new ADRs.
- Proposals in `docs/proposals/` are **read-only requirements baseline** (Flintlock lineage).
- Host provisioning uses **standard iPXE** orchestrated by **DCops** — not Tinkerbell, not Pixiecore-only long term.
- Never commit secrets. Tokens follow `cylon/.env` / SOPS patterns.

## Related wikis

- Cylon: [`../cylon/llmwiki/`](../cylon/llmwiki/) — CRP, resurrection-hub, cylon host
- DCops: [`../DCops/AGENTS.md`](../DCops/AGENTS.md)
