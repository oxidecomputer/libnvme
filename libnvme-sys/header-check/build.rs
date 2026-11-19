// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![deny(warnings)]

use std::{env, path::PathBuf};

extern crate libnvme_sys;

/// Types that are only ever used behind a pointer.
const OPAQUE: &[&str] = &[
    "nvme",
    "nvme_ctrl_iter",
    "nvme_ctrl_disc",
    "nvme_ctrl",
    "nvme_ctrl_info",
    "nvme_ns_iter",
    "nvme_ns_disc",
    "nvme_ns",
    "nvme_ns_info",
    "nvme_log_disc",
    "nvme_log_req",
    "nvme_nvm_lba_fmt",
    "nvme_fw_commit_req",
    "nvme_format_req",
    "di_node",
];

/// Fields of `nvme_identify_ctrl` that are a group of bitfields in C.
const ID_BITFIELDS: &[&str] = &[
    "id_mic",
    "id_oaes",
    "id_ctratt",
    "id_nvmsr",
    "id_vpdwc",
    "id_mec",
    "id_oacs",
    "id_frmw",
    "id_lpa",
    "id_avscc",
    "id_apsta",
    "ap_rpmbs",
    "ap_dsto",
    "ap_hctma",
    "ap_sanitize",
    "ap_anacap",
    "id_sqes",
    "id_cqes",
    "id_oncs",
    "id_fuses",
    "id_fna",
    "id_vwc",
    "id_nvscc",
    "id_nwpc",
    "id_sgls",
];

/// Reserved fields of `nvme_identify_ctrl` that the C header has since split
/// up to make room for new fields.
const ID_STALE_RESERVED: &[&str] = &[
    "id_rsvd_cc",
    "id_rsvd2_cc",
    "id_rsvd_ac",
    "id_rsvd_nc_3",
    "id_rsvd_nc_4",
];

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

    // We need access to sys/nvme.h which is not shipped in /usr/include
    let include_paths = ["usr/src/uts/common"];
    cfg.include("/usr/include");
    for p in include_paths {
        cfg.include(gate_dir.join(p));
    }

    cfg.header("libnvme.h");

    cfg.skip_private(true);
    cfg.skip_struct(|s| OPAQUE.contains(&s.ident()));
    cfg.skip_alias(|t| {
        t.ident().strip_suffix("_t").is_some_and(|name| OPAQUE.contains(&name))
    });

    // These are typedefs of anonymous structs in C.
    cfg.rename_struct_ty(|ty| match ty {
        "nvme_identify_ctrl" => Some("nvme_identify_ctrl_t".to_string()),
        ty if ty.ends_with("_t") => Some(ty.to_string()),
        _ => None,
    });

    // Fields with no C counterpart to compare against.
    cfg.skip_struct_field(|s, f| match s.ident() {
        "nvme_fwslot_log_t" => f.ident() == "bitfield1",
        "nvme_idctl_psd_t" => true,
        "nvme_identify_ctrl" => ID_STALE_RESERVED.contains(&f.ident()),
        _ => false,
    });

    // These are bitfields in C, so only their offset and size can be compared.
    cfg.skip_struct_field_type(|s, f| {
        s.ident() == "nvme_identify_ctrl" && ID_BITFIELDS.contains(&f.ident())
    });

    ctest::generate_test(&mut cfg, "../src/lib.rs", "main.rs").unwrap();
}
