# illumos libnvme bindings

This crate provides a Rust interface to libnvme on illumos systems.

*Warning*: This crate is still under active development, and therefore there is
no API stability.

## Header checks

`libnvme-sys/header-check` and `nvme/header-check` compare the declarations in
this repository against the illumos C headers. They need illumos, and a copy of
illumos-gate, as not every NVMe header is installed.

To run them against the commit of illumos-gate's `stlouis` branch recorded in
`tools/illumos_gate_version`:

```
./tools/header-check.sh
```

That commit is the one the bindings were last verified against. CI checks it,
and separately checks the tip of `stlouis`, so that a failure caused by a change
to illumos can be told apart from one caused by a change here. To move to a
newer illumos, update `tools/illumos_gate_version`.

To check against another revision, pass a full commit hash or a branch:

```
./tools/header-check.sh <rev>
```

To use an existing checkout of illumos-gate instead, run `cargo test` in either
header-check directory with `GATE_SRC` set to its path.
