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

# Check against the commit in tools/illumos_gate_version.
banner pinned
ptime -m ./tools/header-check.sh
