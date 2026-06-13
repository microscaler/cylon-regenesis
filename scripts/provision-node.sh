#!/usr/bin/env bash
# Provision a single resurrection Multipass worker from cylon env + certs.
set -euo pipefail

vm="${1:?usage: provision-node.sh VM INDEX}"
idx="${2:?usage: provision-node.sh VM INDEX}"

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cylon_env="${root}/../cylon/deployment-configuration/profiles/dev/resurrection-node/resurrection-node-${idx}.env"
certs_dir="${root}/../cylon/deployment-configuration/profiles/dev/.certs"

if [[ ! -f "${cylon_env}" ]]; then
  echo "Missing ${cylon_env}" >&2
  exit 1
fi

# shellcheck disable=SC1090
source "${cylon_env}"
rust_log="${RUST_LOG:-info}"

cfg="$(mktemp)"
trap 'rm -f "${cfg}"' EXIT
cat > "${cfg}" <<EOF
REGENESIS_PROVISIONING=node
REGENESIS_NODE_ID=${CYLON_NODE_ID}
REGENESIS_HUB_API=${HUB_API_ENDPOINT}
REGENESIS_GRPC_ENDPOINT=${CYLON_GRPC_ENDPOINT}
REGENESIS_MEMORY_MB=${CYLON_AVAILABLE_MEMORY_MB}
REGENESIS_VCPU=${CYLON_AVAILABLE_VCPU}
REGENESIS_CERTS_DIR=${CYLON_CERTS_DIR}
REGENESIS_RUST_LOG=${rust_log}
REGENESIS_GITHUB_TOKEN_FILE=/etc/cylon/github-token
REGENESIS_RELEASE_PIN_FILE=/etc/cylon/release-pin
EOF

just -f "${root}/justfile" install-to "${vm}"
multipass transfer "${cfg}" "${vm}:/tmp/regenesis-node.env"
multipass exec "${vm}" sudo mv /tmp/regenesis-node.env /etc/regenesis/config.env

if [[ -d "${certs_dir}" ]]; then
  multipass exec "${vm}" sudo mkdir -p "${CYLON_CERTS_DIR}"
  for f in ca.crt server.crt server.key hub-client.crt hub-client.key; do
    if [[ -f "${certs_dir}/${f}" ]]; then
      multipass transfer "${certs_dir}/${f}" "${vm}:/tmp/${f}"
      multipass exec "${vm}" sudo install -m 600 "/tmp/${f}" "${CYLON_CERTS_DIR}/${f}"
      multipass exec "${vm}" sudo rm -f "/tmp/${f}"
    fi
  done
fi

multipass exec "${vm}" sudo env REGENESIS_CYLON_HOST_UNIT=/usr/local/share/regenesis/cylon-host.service \
  /usr/local/bin/regenesis-agent --config /etc/regenesis/config.env --provisioning node --force

echo "Node provisioning OK on ${vm} (${CYLON_NODE_ID})"
