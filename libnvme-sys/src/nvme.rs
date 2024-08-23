// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use bitfield_struct::bitfield;
use std::ffi::{c_char, c_int, c_uint, c_void};

use super::devinfo::di_node;
use super::opaque_type;

pub const NVME_ERR_OK: nvme_err_t = 0;
pub const NVME_ERR_CONTROLLER: nvme_err_t = 1;
pub const NVME_ERR_NO_MEM: nvme_err_t = 2;
pub const NVME_ERR_NO_DMA_MEM: nvme_err_t = 3;
pub const NVME_ERR_LIBDEVINFO: nvme_err_t = 4;
pub const NVME_ERR_INTERNAL: nvme_err_t = 5;
pub const NVME_ERR_BAD_PTR: nvme_err_t = 6;
pub const NVME_ERR_BAD_FLAG: nvme_err_t = 7;
pub const NVME_ERR_BAD_DEVI: nvme_err_t = 8;
pub const NVME_ERR_BAD_DEVI_PROP: nvme_err_t = 9;
pub const NVME_ERR_ILLEGAL_INSTANCE: nvme_err_t = 10;
pub const NVME_ERR_BAD_CONTROLLER: nvme_err_t = 11;
pub const NVME_ERR_PRIVS: nvme_err_t = 12;
pub const NVME_ERR_OPEN_DEV: nvme_err_t = 13;
pub const NVME_ERR_BAD_RESTORE: nvme_err_t = 14;
pub const NVME_ERR_NS_RANGE: nvme_err_t = 15;
pub const NVME_ERR_NS_UNUSE: nvme_err_t = 16;
pub const NVME_ERR_LOG_CSI_RANGE: nvme_err_t = 17;
pub const NVME_ERR_LOG_LID_RANGE: nvme_err_t = 18;
pub const NVME_ERR_LOG_LSP_RANGE: nvme_err_t = 19;
pub const NVME_ERR_LOG_LSI_RANGE: nvme_err_t = 20;
pub const NVME_ERR_LOG_RAE_RANGE: nvme_err_t = 21;
pub const NVME_ERR_LOG_SIZE_RANGE: nvme_err_t = 22;
pub const NVME_ERR_LOG_OFFSET_RANGE: nvme_err_t = 23;
pub const NVME_ERR_LOG_CSI_UNSUP: nvme_err_t = 24;
pub const NVME_ERR_LOG_LSP_UNSUP: nvme_err_t = 25;
pub const NVME_ERR_LOG_LSI_UNSUP: nvme_err_t = 26;
pub const NVME_ERR_LOG_RAE_UNSUP: nvme_err_t = 27;
pub const NVME_ERR_LOG_OFFSET_UNSUP: nvme_err_t = 28;
pub const NVME_ERR_LOG_LSP_UNUSE: nvme_err_t = 29;
pub const NVME_ERR_LOG_LSI_UNUSE: nvme_err_t = 30;
pub const NVME_ERR_LOG_RAE_UNUSE: nvme_err_t = 31;
pub const NVME_ERR_LOG_SCOPE_MISMATCH: nvme_err_t = 32;
pub const NVME_ERR_LOG_REQ_MISSING_FIELDS: nvme_err_t = 33;
pub const NVME_ERR_LOG_NAME_UNKNOWN: nvme_err_t = 34;
pub const NVME_ERR_LOG_UNSUP_BY_DEV: nvme_err_t = 35;
pub const NVME_ERR_IDENTIFY_UNKNOWN: nvme_err_t = 36;
pub const NVME_ERR_IDENTIFY_UNSUP_BY_DEV: nvme_err_t = 37;
pub const NVME_ERR_IDENTIFY_CTRLID_RANGE: nvme_err_t = 38;
pub const NVME_ERR_IDENTIFY_OUTPUT_RANGE: nvme_err_t = 39;
pub const NVME_ERR_IDENTIFY_CTRLID_UNSUP: nvme_err_t = 40;
pub const NVME_ERR_IDENTIFY_CTRLID_UNUSE: nvme_err_t = 41;
pub const NVME_ERR_IDENTIFY_REQ_MISSING_FIELDS: nvme_err_t = 42;
pub const NVME_ERR_VUC_UNSUP_BY_DEV: nvme_err_t = 43;
pub const NVME_ERR_VUC_TIMEOUT_RANGE: nvme_err_t = 44;
pub const NVME_ERR_VUC_OPCODE_RANGE: nvme_err_t = 45;
pub const NVME_ERR_VUC_IMPACT_RANGE: nvme_err_t = 46;
pub const NVME_ERR_VUC_NDT_RANGE: nvme_err_t = 47;
pub const NVME_ERR_VUC_CANNOT_RW: nvme_err_t = 48;
pub const NVME_ERR_VUC_NO_RESULTS: nvme_err_t = 49;
pub const NVME_ERR_VUC_UNKNOWN: nvme_err_t = 50;
pub const NVME_ERR_VUC_REQ_MISSING_FIELDS: nvme_err_t = 51;
pub const NVME_ERR_VU_FUNC_UNSUP_BY_DEV: nvme_err_t = 52;
pub const NVME_ERR_WDC_E6_OFFSET_RANGE: nvme_err_t = 53;
pub const NVME_ERR_FW_UNSUP_BY_DEV: nvme_err_t = 54;
pub const NVME_ERR_KERN_FW_IMPOS: nvme_err_t = 55;
pub const NVME_ERR_FW_LOAD_LEN_RANGE: nvme_err_t = 56;
pub const NVME_ERR_FW_LOAD_OFFSET_RANGE: nvme_err_t = 57;
pub const NVME_ERR_FW_COMMIT_SLOT_RANGE: nvme_err_t = 58;
pub const NVME_ERR_FW_COMMIT_ACTION_RANGE: nvme_err_t = 59;
pub const NVME_ERR_FW_COMMIT_REQ_MISSING_FIELDS: nvme_err_t = 60;
pub const NVME_ERR_FW_SLOT_RO: nvme_err_t = 61;
pub const NVME_ERR_FORMAT_UNSUP_BY_DEV: nvme_err_t = 62;
pub const NVME_ERR_CRYPTO_SE_UNSUP_BY_DEV: nvme_err_t = 63;
pub const NVME_ERR_NS_FORMAT_UNSUP_BY_DEV: nvme_err_t = 64;
pub const NVME_ERR_KERN_FORMAT_UNSUP: nvme_err_t = 65;
pub const NVME_ERR_FORMAT_LBAF_RANGE: nvme_err_t = 66;
pub const NVME_ERR_FORMAT_SES_RANGE: nvme_err_t = 67;
pub const NVME_ERR_FORMAT_PARAM_UNSUP: nvme_err_t = 68;
pub const NVME_ERR_FORMAT_REQ_MISSING_FIELDS: nvme_err_t = 69;
pub const NVME_ERR_WDC_E6_REQ_MISSING_FIELDS: nvme_err_t = 70;
pub const NVME_ERR_FEAT_NAME_UNKNOWN: nvme_err_t = 71;
pub const NVME_ERR_FEAT_UNSUP_BY_DEV: nvme_err_t = 72;
pub const NVME_ERR_FEAT_FID_RANGE: nvme_err_t = 73;
pub const NVME_ERR_FEAT_SEL_RANGE: nvme_err_t = 74;
pub const NVME_ERR_FEAT_CDW11_RANGE: nvme_err_t = 75;
pub const NVME_ERR_FEAT_DATA_RANGE: nvme_err_t = 76;
pub const NVME_ERR_FEAT_SEL_UNSUP: nvme_err_t = 77;
pub const NVME_ERR_FEAT_CDW11_UNUSE: nvme_err_t = 78;
pub const NVME_ERR_FEAT_DATA_UNUSE: nvme_err_t = 79;
pub const NVME_ERR_FEAT_NO_RESULTS: nvme_err_t = 80;
pub const NVME_ERR_GET_FEAT_REQ_MISSING_FIELDS: nvme_err_t = 81;
pub const NVME_ERR_NEED_CTRL_WRLOCK: nvme_err_t = 82;
pub const NVME_ERR_NEED_NS_WRLOCK: nvme_err_t = 83;
pub const NVME_ERR_CTRL_LOCKED: nvme_err_t = 84;
pub const NVME_ERR_NS_LOCKED: nvme_err_t = 85;
pub const NVME_ERR_LOCK_PROG: nvme_err_t = 86;
pub const NVME_ERR_LOCK_ORDER: nvme_err_t = 87;
pub const NVME_ERR_LOCK_WAIT_INTR: nvme_err_t = 88;
pub const NVME_ERR_LOCK_WOULD_BLOCK: nvme_err_t = 89;
pub const NVME_ERR_DETACH_KERN: nvme_err_t = 90;
pub const NVME_ERR_ATTACH_KERN: nvme_err_t = 91;
pub const NVME_ERR_ATTACH_UNSUP_KERN: nvme_err_t = 92;
pub const NVME_ERR_NS_BLKDEV_ATTACH: nvme_err_t = 93;
pub const NVME_ERR_NO_KERN_MEM: nvme_err_t = 94;
pub const NVME_ERR_CTRL_DEAD: nvme_err_t = 95;
pub const NVME_ERR_CTRL_GONE: nvme_err_t = 96;
pub type nvme_err_t = c_uint;

pub const NVME_INFO_ERR_OK: nvme_info_err_t = 0;
pub const NVME_INFO_ERR_TRANSPORT: nvme_info_err_t = 1;
pub const NVME_INFO_ERR_VERSION: nvme_info_err_t = 2;
pub const NVME_INFO_ERR_MISSING_CAP: nvme_info_err_t = 3;
pub const NVME_INFO_ERR_BAD_LBA_FMT: nvme_info_err_t = 4;
pub const NVME_INFO_ERR_PERSIST_NVL: nvme_info_err_t = 5;
pub const NVME_INFO_ERR_BAD_FMT: nvme_info_err_t = 6;
pub const NVME_INFO_ERR_BAD_FMT_DATA: nvme_info_err_t = 7;
pub const NVME_INFO_ERR_NS_INACTIVE: nvme_info_err_t = 8;
pub const NVME_INFO_ERR_NS_NO_BLKDEV: nvme_info_err_t = 9;
pub type nvme_info_err_t = c_uint;

pub const NVME_ITER_VALID: nvme_iter_t = 0;
pub const NVME_ITER_DONE: nvme_iter_t = 1;
pub const NVME_ITER_ERROR: nvme_iter_t = 2;
pub type nvme_iter_t = c_uint;

pub const NVME_LOCK_L_READ: nvme_lock_level_t = 1;
pub const NVME_LOCK_L_WRITE: nvme_lock_level_t = 2;
pub type nvme_lock_level_t = c_uint;

pub const NVME_LOCK_F_DONT_BLOCK: nvme_lock_flags_t = 1;
pub type nvme_lock_flags_t = c_uint;

pub const NVME_NS_DISC_F_ALL: nvme_ns_disc_level_t = 0;
pub const NVME_NS_DISC_F_ALLOCATED: nvme_ns_disc_level_t = 1;
pub const NVME_NS_DISC_F_ACTIVE: nvme_ns_disc_level_t = 2;
pub const NVME_NS_DISC_F_NOT_IGNORED: nvme_ns_disc_level_t = 3;
pub const NVME_NS_DISC_F_BLKDEV: nvme_ns_disc_level_t = 4;
pub type nvme_ns_disc_level_t = ::std::os::raw::c_uint;

pub const NVME_LOG_SIZE_K_UNKNOWN: nvme_log_size_kind_t = 0;
pub const NVME_LOG_SIZE_K_FIXED: nvme_log_size_kind_t = 1;
pub const NVME_LOG_SIZE_K_VAR: nvme_log_size_kind_t = 2;
pub type nvme_log_size_kind_t = c_uint;

// TODO: These come from nvme.h and should probably be pulled out into a NVMe
//  spec crate at some point.
pub const NVME_FWC_SAVE: u32 = 0;
pub const NVME_FWC_SAVE_ACTIVATE: u32 = 1;
pub const NVME_FWC_ACTIVATE: u32 = 2;
pub const NVME_FWC_ACTIVATE_IMMED: u32 = 3;

const NVME_FWVER_SZ: usize = 8;

#[bitfield(u8)]
pub struct nvme_fwslot_log_t_bitfield1 {
    /// Active Firmware Slot
    #[bits(3, access = RO)]
    pub fw_afi: u8,
    #[bits(1)]
    __: u8, // fw_rsvd1
    /// Next Active Firmware Slot
    #[bits(3, access = RO)]
    pub fw_next: u8,
    #[bits(1)]
    /// fw_rsvd2
    __: B1,
}

#[repr(C)]
#[derive(Debug)]
pub struct nvme_fwslot_log_t {
    // fw_afi and fw_next
    pub bitfield1: nvme_fwslot_log_t_bitfield1,
    _reserved1: [u8; 7],
    pub fw_frs: [[c_char; NVME_FWVER_SZ]; 7],
    _reserved2: [u8; 512 - 64],
}

opaque_type!(nvme, nvme_t);
opaque_type!(nvme_ctrl_iter, nvme_ctrl_iter_t);
opaque_type!(nvme_ctrl_disc, nvme_ctrl_disc_t);
opaque_type!(nvme_ctrl, nvme_ctrl_t);
opaque_type!(nvme_ctrl_info, nvme_ctrl_info_t);
opaque_type!(nvme_ns_iter, nvme_ns_iter_t);
opaque_type!(nvme_ns_disc, nvme_ns_disc_t);
opaque_type!(nvme_ns, nvme_ns_t);
opaque_type!(nvme_ns_info, nvme_ns_info_t);
opaque_type!(nvme_nvm_lba_fmt, nvme_nvm_lba_fmt_t);
opaque_type!(nvme_format_req, nvme_format_req_t);
opaque_type!(nvme_log_disc, nvme_log_disc_t);
opaque_type!(nvme_log_req, nvme_log_req_t);
opaque_type!(nvme_fw_commit_req, nvme_fw_commit_req_t);

// Using "super" here rather than "crate" because `ctest2` does not support rust
// 2018 edition.
pub type nvme_identify_ctrl_t = super::identify::nvme_identify_ctrl;

#[link(name = "nvme")]
extern "C" {
    // NVMe handle.
    pub fn nvme_init() -> *mut nvme_t;
    pub fn nvme_fini(nvme: *mut nvme_t);

    /// NVMe Handle errors.
    pub fn nvme_err(nvme: *mut nvme_t) -> nvme_err_t;
    pub fn nvme_errmsg(nvme: *mut nvme_t) -> *const c_char;
    pub fn nvme_syserr(nvme: *mut nvme_t) -> c_int;

    // NVMe Controller errors.
    pub fn nvme_ctrl_err(ctrl: *mut nvme_ctrl_t) -> nvme_err_t;
    pub fn nvme_ctrl_errmsg(ctrl: *mut nvme_ctrl_t) -> *const c_char;
    pub fn nvme_ctrl_syserr(ctrl: *mut nvme_ctrl_t) -> c_int;

    pub fn nvme_ctrl_deverr(
        ctrl: *mut nvme_ctrl_t,
        sct: *mut u32,
        sc: *mut u32,
    );

    // NVMe Controller discovery.
    pub fn nvme_ctrl_disc_devi(discp: *const nvme_ctrl_disc_t) -> *mut di_node;
    pub fn nvme_ctrl_discover_init(
        nvme: *mut nvme_t,
        iterp: *mut *mut nvme_ctrl_iter_t,
    ) -> bool;
    pub fn nvme_ctrl_discover_step(
        iter: *mut nvme_ctrl_iter_t,
        discp: *mut *const nvme_ctrl_disc_t,
    ) -> nvme_iter_t;
    pub fn nvme_ctrl_discover_fini(iter: *mut nvme_ctrl_iter_t);
    pub fn nvme_ctrl_init(
        nvme: *mut nvme_t,
        di: *mut di_node,
        outp: *mut *mut nvme_ctrl_t,
    ) -> bool;
    pub fn nvme_ctrl_init_by_instance(
        nvme: *mut nvme_t,
        inst: i32,
        outp: *mut *mut nvme_ctrl_t,
    ) -> bool;
    pub fn nvme_ctrl_fini(ctrl: *mut nvme_ctrl_t);

    // NVMe Controller information. Information about a controller is a
    // separate lifetime than the controller itself.
    pub fn nvme_ctrl_info_snap(
        ctrl: *mut nvme_ctrl_t,
        outp: *mut *mut nvme_ctrl_info_t,
    ) -> bool;
    pub fn nvme_ctrl_info_free(ci: *mut nvme_ctrl_info_t);

    // NVMe Controller information errors.
    pub fn nvme_ctrl_info_err(ci: *mut nvme_ctrl_info_t) -> nvme_info_err_t;
    pub fn nvme_ctrl_info_errmsg(ci: *mut nvme_ctrl_info_t) -> *const c_char;
    pub fn nvme_ctrl_info_syserr(ci: *mut nvme_ctrl_info_t) -> c_int;

    // Information aobut an NVMe Controller
    pub fn nvme_ctrl_info_identify(
        ci: *mut nvme_ctrl_info_t,
    ) -> *const nvme_identify_ctrl_t;
    pub fn nvme_ctrl_info_model(ci: *mut nvme_ctrl_info_t) -> *const c_char;
    pub fn nvme_ctrl_info_serial(ci: *mut nvme_ctrl_info_t) -> *const c_char;
    pub fn nvme_ctrl_info_fwrev(ci: *mut nvme_ctrl_info_t) -> *const c_char;
    pub fn nvme_ctrl_info_nns(ci: *mut nvme_ctrl_info_t) -> u32;
    pub fn nvme_ctrl_info_pci_vid(
        ci: *mut nvme_ctrl_info_t,
        u16p: *mut u16,
    ) -> bool;

    // NVM command set for controllers.
    pub fn nvme_ctrl_info_nformats(ci: *mut nvme_ctrl_info_t) -> u32;
    pub fn nvme_ctrl_info_format(
        ci: *mut nvme_ctrl_info_t,
        idx: u32,
        outp: *mut *const nvme_nvm_lba_fmt_t,
    ) -> bool;
    pub fn nvme_nvm_lba_fmt_id(labf: *const nvme_nvm_lba_fmt_t) -> u32;
    pub fn nvme_nvm_lba_fmt_meta_size(labf: *const nvme_nvm_lba_fmt_t) -> u32;
    pub fn nvme_nvm_lba_fmt_data_size(labf: *const nvme_nvm_lba_fmt_t) -> u64;
    pub fn nvme_nvm_lba_fmt_rel_perf(labf: *const nvme_nvm_lba_fmt_t) -> u32;

    // NVMe Namespace Discovery.
    pub fn nvme_ns_discover_init(
        ctrl: *mut nvme_ctrl_t,
        level: nvme_ns_disc_level_t,
        iter: *mut *mut nvme_ns_iter_t,
    ) -> bool;
    pub fn nvme_ns_discover_fini(iter: *mut nvme_ns_iter_t);
    pub fn nvme_ns_discover_step(
        iter: *mut nvme_ns_iter_t,
        discp: *mut *const nvme_ns_disc_t,
    ) -> nvme_iter_t;
    pub fn nvme_ns_disc_nsid(discp: *const nvme_ns_disc_t) -> u32;
    pub fn nvme_ns_init(
        ctrl: *mut nvme_ctrl_t,
        nsid: u32,
        nsp: *mut *mut nvme_ns_t,
    ) -> bool;
    pub fn nvme_ns_fini(ns: *mut nvme_ns_t);

    // NVMe Namespace information.
    pub fn nvme_ns_info_snap(
        ns: *mut nvme_ns_t,
        infop: *mut *mut nvme_ns_info_t,
    ) -> bool;
    pub fn nvme_ns_info_free(info: *mut nvme_ns_info_t);
    pub fn nvme_ns_info_err(info: *mut nvme_ns_info_t) -> nvme_info_err_t;
    pub fn nvme_ns_info_errmsg(info: *mut nvme_ns_info_t) -> *const c_char;
    pub fn nvme_ns_info_syserr(info: *mut nvme_ns_info_t) -> c_int;
    pub fn nvme_ns_info_curformat(
        info: *mut nvme_ns_info_t,
        fmtp: *mut *const nvme_nvm_lba_fmt,
    ) -> bool;

    // Controller Locking.
    pub fn nvme_ctrl_lock(
        ctrl: *mut nvme_ctrl_t,
        level: nvme_lock_level_t,
        flags: nvme_lock_flags_t,
    ) -> bool;
    pub fn nvme_ctrl_unlock(ctrl: *mut nvme_ctrl_t);

    // Namespace Attach and Detach.
    pub fn nvme_ns_bd_attach(ns: *mut nvme_ns_t) -> bool;
    pub fn nvme_ns_bd_detach(ns: *mut nvme_ns_t) -> bool;

    // NVMe Log Page Discovery
    pub fn nvme_log_req_init_by_name(
        ctrl: *mut nvme_ctrl_t,
        name: *const c_char,
        flags: c_uint,
        discp: *mut *mut nvme_log_disc_t,
        reqp: *mut *mut nvme_log_req_t,
    ) -> bool;
    pub fn nvme_log_disc_size(
        disc: *const nvme_log_disc_t,
        sizep: *mut u64,
    ) -> nvme_log_size_kind_t;
    pub fn nvme_log_req_set_output(
        req: *mut nvme_log_req_t,
        buf: *mut c_void,
        buflen: usize,
    ) -> bool;
    pub fn nvme_log_disc_calc_size(
        disc: *const nvme_log_disc_t,
        act: *mut u64,
        buf: *const c_void,
        buflen: usize,
    ) -> bool;
    pub fn nvme_log_req_exec(req: *mut nvme_log_req_t) -> bool;
    pub fn nvme_log_disc_free(disc: *mut nvme_log_disc_t);
    pub fn nvme_log_req_fini(req: *mut nvme_log_req_t);

    // Firmware Download and Commit (Activation)
    pub fn nvme_fw_load(
        ctrl: *mut nvme_ctrl_t,
        buf: *const c_void,
        len: usize,
        off: u64,
    ) -> bool;
    pub fn nvme_fw_commit_req_init(
        ctrl: *mut nvme_ctrl_t,
        reqp: *mut *mut nvme_fw_commit_req_t,
    ) -> bool;
    pub fn nvme_fw_commit_req_fini(req: *mut nvme_fw_commit_req_t);
    pub fn nvme_fw_commit_req_set_slot(
        req: *mut nvme_fw_commit_req_t,
        slot: u32,
    ) -> bool;
    pub fn nvme_fw_commit_req_set_action(
        req: *mut nvme_fw_commit_req_t,
        act: u32,
    ) -> bool;
    pub fn nvme_fw_commit_req_exec(req: *mut nvme_fw_commit_req_t) -> bool;

    // Format NVM
    //
    // These are used to erase and reformat either all namespaces or a specific
    // one.
    pub fn nvme_format_req_init(
        ctrl: *mut nvme_ctrl_t,
        reqp: *mut *mut nvme_format_req_t,
    ) -> bool;
    pub fn nvme_format_req_set_lbaf(
        req: *mut nvme_format_req_t,
        lbaf: u32,
    ) -> bool;
    pub fn nvme_format_req_set_ses(
        req: *mut nvme_format_req_t,
        ses: u32,
    ) -> bool;
    pub fn nvme_format_req_set_nsid(
        req: *mut nvme_format_req_t,
        nsid: u32,
    ) -> bool;
    pub fn nvme_format_req_exec(req: *mut nvme_format_req_t) -> bool;
    pub fn nvme_format_req_fini(req: *mut nvme_format_req_t);

    // WDC resizing functions.  These are interfaces supported in the SN840,
    // SN650, SN655, etc.
    pub fn nvme_wdc_resize_set(ctrl: *mut nvme_ctrl_t, gb: u32) -> bool;
    pub fn nvme_wdc_resize_get(ctrl: *mut nvme_ctrl_t, gbp: *mut u32) -> bool;

}
