// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![deny(warnings)]

use std::{env, fs, path::PathBuf};

extern crate nvme;

fn main() {
    // This is a bit of a hack, but we want this to always rerun as the users
    // GATE_SRC checkout may have changed.
    println!("cargo:rerun-if-changed=./target");

    let mut cfg = ctest::TestGenerator::new();

    // We cannot proceed without a path to the source
    let gate_dir = match env::var("GATE_SRC").map(PathBuf::try_from) {
        Ok(Ok(dir)) => dir,
        _ => {
            eprintln!("Must specify path to illumos-gate sources with GATE_SRC env var");
            std::process::exit(1);
        }
    };

    // sys/nvme.h is not shipped in /usr/include. Copy it out of the gate, so
    // that none of the gate's other headers are picked up with it.
    let headers = PathBuf::from(env::var("OUT_DIR").unwrap()).join("include");
    let from = gate_dir.join("usr/src/uts/common/sys/nvme.h");
    let _ = fs::remove_dir_all(&headers);
    fs::create_dir_all(headers.join("sys")).unwrap();
    fs::copy(&from, headers.join("sys/nvme.h"))
        .unwrap_or_else(|e| panic!("failed to copy {}: {e}", from.display()));
    cfg.include(&headers);
    cfg.include("/usr/include");

    cfg.header("sys/nvme.h");
    ctest::generate_test(&mut cfg, "../src/lib.rs", "main.rs").unwrap();
}
