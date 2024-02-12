// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![deny(warnings)]

extern crate nvme_sys;

fn main() {
    let mut cfg = ctest2::TestGenerator::new();

    cfg.include("/usr/include");
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
        "nvme_nvm_lba_fmt" => true,
        "nvme_format_req" => true,
        "di_node" => true,
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
        "nvme_nvm_lba_fmt_t" => true,
        "nvme_format_req_t" => true,
        "di_node_t" => true,
        _ => false,
    });

    cfg.generate("../src/lib.rs", "main.rs");
}
