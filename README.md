# Cylon Regenesis — deprecated

> **Deprecated:** 2026-07-28
>
> **Replacement:** [`microscaler/Aether`](https://github.com/microscaler/Aether)
>
> **Status:** frozen pending archival

This repository is no longer an active control-plane, host-management, or
development-environment project. Do not add new features or deploy its
Multipass controller, `regenesis-agent`, iPXE copy, or proposed Regenesis Hub.

The ownership decision is:

| Concern | Authoritative repository |
|---|---|
| Placement, node lifecycle, direct QEMU/KVM, storage, fencing and recovery | `microscaler/Aether` |
| Dockerfile-built KVM guest images and kernels | `microscaler/cylon-images` |
| Guest skills and persona tooling | `microscaler/cylon-skills` |
| DHCP, IPAM, BootIntent and iPXE delivery | `microscaler/DCops` |
| Cognitive workload semantics | `microscaler/cylon` |

Aether's primary runtime is native QEMU/KVM. Guest filesystems are assembled by
Docker builds in `cylon-images`, packaged as OCI-distributed KVM bundles, and
materialized directly by `aetherd`. Multipass and Regenesis are not part of
that path.

## Repository contents

The Rust Multipass controller, shell provisioning agent, design plans, ADRs and
runbooks are retained as historical evidence only. They are not supported
components and must not be copied wholesale into Aether. In particular:

- the proposed Raft-backed `regenesis-hub` was never implemented and is
  superseded by Aether;
- the Multipass controller was never wired into a deployed operational path;
- direct Resurrection Hub registration, drain and watchdog contracts are
  obsolete; and
- the iPXE artifact is already owned and served by DCops.

See [the deprecation and archive plan](docs/DEPRECATION.md) and
[ADR-0007](adrs/ADR-0007-deprecate-in-favour-of-aether-native-kvm.md).

## Archive gate

The GitHub repository can be archived after:

1. active sibling repositories contain no build or runtime dependency on
   `cylon-regenesis`;
2. the Dockerfile-to-KVM artifact builds and validates in `cylon-images`;
3. an Aether host boots that artifact with native KVM and completes its
   registration, observation and teardown lifecycle; and
4. a final read-only tag records this repository state.

Legacy `just` provisioning recipes now fail with a deprecation message. This is
intentional: there is no supported Multipass continuation path.
