# Phase 3 — Control plane extraction

**Depends on:** Phase 1 (functional agent path)  
**Goal:** Move `tiffany/crates/resurrection-hub` → `cylon-regenesis/crates/regenesis-hub`.

## Work items

### 3.1 Rust workspace bootstrap

```
cylon-regenesis/
├── Cargo.toml          # workspace
├── crates/
│   └── regenesis-hub/
├── rust-toolchain.toml
└── justfile
```

### 3.2 Git history

Prefer `git filter-repo` or copy with attribution note in CHANGELOG — preserve blame where possible.

### 3.3 Dependency on cylon proto

| Option | Tradeoff |
|---|---|
| Path dep `../../tiffany/crates/cylon` | Simple dev |
| Published `cylon-grpc` crate | Clean boundary |

Document pin in ARCHITECTURE version matrix.

### 3.4 Tiffany integration

- Tilt builds `regenesis-hub` from cylon-regenesis path (Tiltfile change)
- Remove duplicate crate from tiffany workspace OR re-export as dependency
- Update `llmwiki` references

### 3.5 CI

- GitHub Actions: `cargo nextest`, clippy, docker publish `ghcr.io/microscaler/regenesis-hub`

### 3.6 API contract tests

- Pact or integration tests against `/v2/*` OpenAPI

## Acceptance criteria

- [ ] Kind deployment uses cylon-regenesis image only
- [ ] All existing resurrection-hub tests pass
- [ ] Tiffany CI green with hub removed from workspace
- [ ] No duplicate hub code in tiffany

## Rollback

- Tilt pin back to tiffany crate path
- Single revert commit

## Next phase

[Phase 4 — Fault tolerance](phase-4-fault-tolerance.md)
