# Cylon Regenesis is frozen pending archival.

default:
    @echo "cylon-regenesis is deprecated; use Aether native KVM" >&2
    @exit 2

install-to vm:
    @echo "install-to {{vm}} retired: Multipass/Regenesis provisioning is unsupported" >&2
    @exit 2

provision-base vm="resurrection-node-base":
    @echo "provision-base {{vm}} retired: build an OCI KVM image in cylon-images" >&2
    @exit 2

provision-node vm node_index:
    @echo "provision-node {{vm}}/{{node_index}} retired: use Aether native KVM" >&2
    @exit 2

provision-fleet:
    @echo "provision-fleet retired: there is no supported Multipass fleet" >&2
    @exit 2
