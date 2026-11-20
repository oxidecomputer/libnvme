#!/bin/bash
#
# Run the header-check crates against a revision of illumos-gate.
#
# usage: header-check.sh [<rev>]
#
# <rev> is a full commit hash or a branch of illumos-gate. It defaults to the
# commit in tools/illumos_gate_version.

set -o errexit
set -o pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
# shellcheck source=tools/illumos_gate_version
source "$root/tools/illumos_gate_version"
rev=${1:-$COMMIT}

gate=$(mktemp -d)
trap 'rm -rf "$gate"' EXIT

git init --quiet "$gate"
git -C "$gate" remote add origin https://code.oxide.computer/illumos-gate
git -C "$gate" sparse-checkout set --cone \
	usr/src/lib/libnvme/common \
	usr/src/uts/common/sys
git -C "$gate" fetch --quiet --depth 1 origin "$rev"
git -C "$gate" checkout --quiet FETCH_HEAD
echo "checking against illumos-gate $(git -C "$gate" rev-parse HEAD)"

export GATE_SRC=$gate
for dir in libnvme-sys/header-check nvme/header-check; do
	echo "running $dir"
	(cd "$root/$dir" && cargo test --locked)
done
