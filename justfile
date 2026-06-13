# Cylon Regenesis — just recipes

regenesis_dir := justfile_directory()
scripts_dir := regenesis_dir + "/scripts"
config_dir := regenesis_dir + "/config"
systemd_dir := regenesis_dir + "/systemd"
ms02_lan := env_var_or_default("MS02_LAN_IP", "192.168.1.189")

default:
	@just --list

# Install agent + units onto a Multipass VM (base or worker).
install-to vm:
	#!/usr/bin/env bash
	set -euo pipefail
	vm="{{vm}}"
	multipass transfer "{{scripts_dir}}/regenesis-agent" "${vm}:/tmp/regenesis-agent"
	multipass transfer "{{systemd_dir}}/cylon-host.service" "${vm}:/tmp/cylon-host.service"
	multipass transfer "{{systemd_dir}}/regenesis-agent.service" "${vm}:/tmp/regenesis-agent.service"
	multipass exec "${vm}" -- sudo install -m 755 /tmp/regenesis-agent /usr/local/bin/regenesis-agent
	multipass exec "${vm}" -- sudo mkdir -p /usr/local/share/regenesis /etc/regenesis
	multipass exec "${vm}" -- sudo install -m 644 /tmp/cylon-host.service /usr/local/share/regenesis/cylon-host.service
	multipass exec "${vm}" -- sudo install -m 644 /tmp/regenesis-agent.service /etc/systemd/system/regenesis-agent.service
	multipass exec "${vm}" -- sudo rm -f /tmp/regenesis-agent /tmp/cylon-host.service /tmp/regenesis-agent.service
	echo "Installed regenesis-agent on ${vm}"

# Run base provisioning on resurrection-node-base (after minimal cloud-init).
provision-base vm="resurrection-node-base":
	just install-to {{vm}}
	#!/usr/bin/env bash
	set -euo pipefail
	vm="{{vm}}"
	multipass transfer "{{config_dir}}/regenesis-base.env" "${vm}:/tmp/regenesis-base.env"
	multipass exec "${vm}" -- sudo mv /tmp/regenesis-base.env /etc/regenesis/config.env
	multipass exec "${vm}" -- sudo REGENESIS_CYLON_HOST_UNIT=/usr/local/share/regenesis/cylon-host.service \
		/usr/local/bin/regenesis-agent --config /etc/regenesis/config.env --provisioning base
	multipass exec "${vm}" -- bash -lc 'test -x /usr/local/bin/cylon && firecracker --version && test -f /home/cylon/cylon-images/vmlinux'
	echo "Base provisioning OK on ${vm}"

# Generate node config.env from cylon resurrection-node-N.env and provision worker.
provision-node vm node_index:
	"{{scripts_dir}}/provision-node.sh" "{{vm}}" "{{node_index}}"

# Provision resurrection-node-1..3 from cylon env files.
provision-fleet:
	just provision-node resurrection-node-1 1
	just provision-node resurrection-node-2 2
	just provision-node resurrection-node-3 3
