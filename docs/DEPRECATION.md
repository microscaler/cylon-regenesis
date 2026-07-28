# Deprecation and archive plan

**Decision date:** 2026-07-28

**State:** deprecated, read-only except for retirement work

## Decision

Aether is the sole infrastructure lifecycle authority. Development and
production converge on direct QEMU/KVM managed by `aetherd`; Multipass is not a
stepping stone or supported test environment. Docker remains valuable only as
the declarative build frontend for guest filesystems, which `cylon-images`
packages into Aether's versioned KVM OCI bundle.

No Regenesis implementation is migrated wholesale:

| Regenesis asset | Retirement action |
|---|---|
| Proposed `regenesis-hub` control plane | Superseded by Aether; do not implement |
| `cylon-multipass-controller` | Freeze as an unwired prototype; do not transplant |
| `regenesis-agent` | Retire; Aether host installation is a separate direct-bare-metal concern |
| Multipass/cloud-init recipes | Retire without replacement |
| iPXE script | DCops is authoritative; remove Regenesis namespace references there |
| Cylon host unit/install snippets | Cylon owns its own packaging |
| Two-kernel and upgrade explanations | Preserve only where still relevant in Aether/Cylon Images docs |

## Replacement path

1. `cylon-images/container/rootfs/ubuntu/Dockerfile` composes the guest OS and
   compatible `cylon-skills` content.
2. `cylon-images/container/kvm` converts that filesystem to an immutable qcow2
   base and publishes the disk, kernel, initramfs and `image.json` via OCI.
3. An Aether workload specifies `runtime_class = "aether-kvm"` with an
   `oci://` image URI.
4. `aetherd` verifies and caches the OCI bundle by digest, creates a writable
   per-VM overlay, attaches a small NoCloud `cidata` drive, and boots QEMU/KVM
   directly.
5. Aether owns observation, teardown, fencing and recovery.

## Archive checklist

- [x] Disable automated Regenesis packaging and GitHub releases.
- [ ] Remove calls to `../cylon-regenesis` from active sibling repositories.
- [ ] Remove `/cylon-regenesis/` artifact URLs and `regenesis.hub` boot
      arguments from DCops.
- [ ] Publish and inspect an `aether-kvm-ubuntu` artifact.
- [ ] Boot the artifact on a real `/dev/kvm` host and verify Cloud-Init and QEMU
      Guest Agent readiness.
- [ ] Exercise Aether ensure → observe → teardown and failure recovery.
- [ ] Search every active repository for runtime/build references to
      `cylon-regenesis`, `regenesis-agent`, `MultipassNode`, and
      `resurrection-nodes-finish-regenesis`.
- [ ] Create a final release/tag, set the GitHub repository description to
      deprecated, and archive it read-only.

Historical documentation may retain those terms when clearly labelled as
superseded. It must not be used as an active roadmap.
