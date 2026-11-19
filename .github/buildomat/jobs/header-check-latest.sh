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

# Check against the tip of stlouis. If this fails while the header-check job
# passes, illumos has changed: update the bindings, then
# tools/illumos_gate_version.
banner latest
ptime -m ./tools/header-check.sh stlouis
