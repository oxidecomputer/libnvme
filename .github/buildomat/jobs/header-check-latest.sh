#!/bin/bash
#:
#: name = "header-check-latest"
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

# A failure here that header-check does not share means illumos has changed,
# and the bindings and tools/illumos_gate_version need to catch up.
banner latest
ptime -m ./tools/header-check.sh stlouis
