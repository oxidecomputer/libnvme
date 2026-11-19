#!/bin/bash
#:
#: name = "header-check"
#: variety = "basic"
#: target = "helios-latest"
#: rust_toolchain = "stable"
#: output_rules = []
#:

set -o errexit
set -o pipefail
set -o xtrace

cargo --version
rustc --version

banner gate
# sys/nvme.h is not shipped in /usr/include, so take it from illumos-gate.
git clone --depth 1 --sparse --branch stlouis \
	https://code.oxide.computer/illumos-gate /work/gate
git -C /work/gate sparse-checkout set usr/src/uts/common/sys
export GATE_SRC=/work/gate

for dir in libnvme-sys/header-check nvme/header-check; do
	banner "$(dirname "$dir")"
	(cd "$dir" && ptime -m cargo test --locked)
done
