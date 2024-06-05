// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![deny(warnings)]

use std::{env, path::PathBuf};

extern crate nvme;

fn main() {
    // This is a bit of a hack, but we want this to always rerun as the users
    // GATE_SRC checkout may have changed.
    println!("cargo:rerun-if-changed=./target");

    let mut cfg = ctest2::TestGenerator::new();

    // We cannot proceed without a path to the source
    let gate_dir = match env::var("GATE_SRC").map(PathBuf::try_from) {
        Ok(Ok(dir)) => dir,
        _ => {
            eprintln!("Must specify path to illumos-gate sources with GATE_SRC env var");
            std::process::exit(1);
        }
    };

    // We need access to sys/nvme.h which is not shipped in /usr/include
    let include_paths = ["usr/src/uts/common"];
    cfg.include("/usr/include");
    for p in include_paths {
        cfg.include(gate_dir.join(p));
    }

    cfg.header("sys/nvme.h");
    cfg.generate("../src/lib.rs", "main.rs");
}
