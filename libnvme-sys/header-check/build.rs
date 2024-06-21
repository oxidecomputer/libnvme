// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![deny(warnings)]

use std::{env, path::PathBuf};

extern crate libnvme_sys;

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

    cfg.header("libnvme.h");

    cfg.skip_struct(|name| match name {
        // Skip over the opaque types
        "nvme" => true,
        "nvme_ctrl_iter" => true,
        "nvme_ctrl_disc" => true,
        "nvme_ctrl" => true,
        "nvme_ctrl_info" => true,
        "nvme_ns_iter" => true,
        "nvme_ns_disc" => true,
        "nvme_ns" => true,
        "nvme_ns_info" => true,
        "nvme_log_disc" => true,
        "nvme_log_req" => true,
        "nvme_nvm_lba_fmt" => true,
        "nvme_fw_commit_req" => true,
        "nvme_format_req" => true,
        "di_node" => true,

        // Skip over u128
        "nvme_uint128_t" => true,

        // Skip over bitfield structs
        "nvme_fwslot_log_t" => true,
        "nvme_idctl_qes_t" => true,
        "nvme_idctl_psd_t" => true,
        "nvme_idctl_psd_t_chunk_1" => true,
        "nvme_idctl_psd_t_chunk_2" => true,
        "nvme_idctl_psd_t_chunk_3" => true,
        "nvme_fwslot_log_t_bitfield1" => true,
        "IdMic" => true,
        "IdOaes" => true,
        "IdCtratt" => true,
        "IdNvmsr" => true,
        "IdVpdwc" => true,
        "IdMec" => true,
        "IdOacs" => true,
        "IdFrmw" => true,
        "IdLpa" => true,
        "IdAvscc" => true,
        "IdApsta" => true,
        "ApRpmbs" => true,
        "ApDsto" => true,
        "ApHctma" => true,
        "ApSanitize" => true,
        "ApAnacap" => true,
        "IdOncs" => true,
        "IdFuses" => true,
        "IdFna" => true,
        "IdVwc" => true,
        "IdNvscc" => true,
        "IdNwpc" => true,
        "IdSgls" => true,
        "nvme_identify_ctrl" => true,

        _ => false,
    });

    cfg.skip_type(|name| match name {
        // Skip over the opaque types
        "nvme_t" => true,
        "nvme_ctrl_iter_t" => true,
        "nvme_ctrl_disc_t" => true,
        "nvme_ctrl_t" => true,
        "nvme_ctrl_info_t" => true,
        "nvme_ns_iter_t" => true,
        "nvme_ns_disc_t" => true,
        "nvme_ns_t" => true,
        "nvme_ns_info_t" => true,
        "nvme_log_disc_t" => true,
        "nvme_log_req_t" => true,
        "nvme_nvm_lba_fmt_t" => true,
        "nvme_fw_commit_req_t" => true,
        "nvme_format_req_t" => true,
        "di_node_t" => true,

        // Skip over u128
        "nvme_uint128_t" => true,

        // Skip over bitfield structs
        "nvme_fwslot_log_t" => true,
        "nvme_idctl_qes_t" => true,
        "nvme_idctl_psd_t" => true,
        "nvme_idctl_psd_t_chunk_1" => true,
        "nvme_idctl_psd_t_chunk_2" => true,
        "nvme_idctl_psd_t_chunk_3" => true,
        "nvme_fwslot_log_t_bitfield1" => true,
        "IdMic" => true,
        "IdOaes" => true,
        "IdCtratt" => true,
        "IdNvmsr" => true,
        "IdVpdwc" => true,
        "IdMec" => true,
        "IdOacs" => true,
        "IdFrmw" => true,
        "IdLpa" => true,
        "IdAvscc" => true,
        "IdApsta" => true,
        "ApRpmbs" => true,
        "ApDsto" => true,
        "ApHctma" => true,
        "ApSanitize" => true,
        "ApAnacap" => true,
        "IdOncs" => true,
        "IdFuses" => true,
        "IdFna" => true,
        "IdVwc" => true,
        "IdNvscc" => true,
        "IdNwpc" => true,
        "IdSgls" => true,
        "nvme_identify_ctrl" => true,
        "nvme_identify_ctrl_t" => true,

        _ => false,
    });

    cfg.generate("../src/lib.rs", "main.rs");
}
