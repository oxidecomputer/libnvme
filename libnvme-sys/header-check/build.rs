// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![deny(warnings)]

use std::{
    env, fs,
    path::{Path, PathBuf},
};

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
    "nvme_log_iter",
    "nvme_log_disc",
    "nvme_log_req",
    "nvme_nvm_lba_fmt",
    "nvme_fw_commit_req",
    "nvme_format_req",
    "di_node",
];

/// Fields of `nvme_identify_ctrl` that are a struct in C but an integer or
/// byte array here, so only their offset and size can be compared.
const ID_STRUCTS: &[&str] = &[
    "id_mic",
    "id_oaes",
    "id_ctratt",
    "id_bpcap",
    "id_plsi",
    "id_crcap",
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
    "ap_kpioc",
    "ap_tmpthha",
    "ap_mupa",
    "ap_cdpa",
    "ap_impsr",
    "id_sqes",
    "id_cqes",
    "id_oncs",
    "id_fuses",
    "id_fna",
    "id_vwc",
    "id_nvscc",
    "id_nwpc",
    "id_cdfs",
    "id_sgls",
    "id_trattr",
];

/// Fields of `nvme_health_log_t` that are a struct in C but an integer here.
const HEALTH_STRUCTS: &[&str] = &["hl_crit_warn", "hl_ecgcws", "hl_ipm"];

/// Fields of `nvme_idctl_psd_t` that are one or more bitfields in C.
const PSD_BITFIELDS: &[&str] = &[
    "psd_mps_nops_bits",
    "psd_rrt_bits",
    "psd_rrl_bits",
    "psd_rwt_bits",
    "psd_rwl_bits",
    "psd_ips_bits",
    "psd_apw_aps_bits",
    "psd_epfrts_fqvts_bits",
    "psd_epfvts_bits",
    "psd_mbws_bits",
];

/// Copy the NVMe headers out of an illumos-gate checkout into `dest`, laid out
/// as they are installed.
///
/// `dest` can then go ahead of /usr/include, so that every NVMe header comes
/// from the one revision of the gate while every other header is the
/// system's.
fn copy_nvme_headers(gate: &Path, dest: &Path) {
    let copy = |from: PathBuf, to: PathBuf| {
        fs::copy(&from, &to).unwrap_or_else(|e| {
            panic!("failed to copy {}: {e}", from.display())
        });
    };
    let sys = gate.join("usr/src/uts/common/sys");

    let _ = fs::remove_dir_all(dest);
    fs::create_dir_all(dest.join("sys/nvme")).unwrap();
    copy(
        gate.join("usr/src/lib/libnvme/common/libnvme.h"),
        dest.join("libnvme.h"),
    );
    copy(sys.join("nvme.h"), dest.join("sys/nvme.h"));
    let vendor = sys.join("nvme");
    let entries = fs::read_dir(&vendor)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", vendor.display()));
    for entry in entries {
        let entry = entry.unwrap();
        copy(entry.path(), dest.join("sys/nvme").join(entry.file_name()));
    }
}

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

    let headers = PathBuf::from(env::var("OUT_DIR").unwrap()).join("include");
    copy_nvme_headers(&gate_dir, &headers);
    cfg.include(&headers);
    cfg.include("/usr/include");

    cfg.header("libnvme.h");
    cfg.header("sys/nvme/wdc_sn840.h");

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
        "nvme_fwslot_log_t" => f.ident() == "fw_afi_next_bits",
        "nvme_idctl_psd_t" => PSD_BITFIELDS.contains(&f.ident()),
        _ => false,
    });

    // Only the offset and size of these can be compared.
    cfg.skip_struct_field_type(|s, f| match s.ident() {
        "nvme_identify_ctrl" => ID_STRUCTS.contains(&f.ident()),
        "nvme_health_log_t" => HEALTH_STRUCTS.contains(&f.ident()),
        _ => false,
    });

    ctest::generate_test(&mut cfg, "../src/lib.rs", "main.rs").unwrap();
}
