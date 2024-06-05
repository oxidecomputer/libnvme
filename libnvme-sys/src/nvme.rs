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
    pub fn nvme_fini(arg1: *mut nvme_t);

    /// NVMe Handle errors.
    pub fn nvme_err(arg1: *mut nvme_t) -> nvme_err_t;
    pub fn nvme_errmsg(arg1: *mut nvme_t) -> *const c_char;
    pub fn nvme_syserr(arg1: *mut nvme_t) -> c_int;

    // NVMe Controller errors.
    pub fn nvme_ctrl_err(arg1: *mut nvme_ctrl_t) -> nvme_err_t;
    pub fn nvme_ctrl_errmsg(arg1: *mut nvme_ctrl_t) -> *const c_char;
    pub fn nvme_ctrl_syserr(arg1: *mut nvme_ctrl_t) -> c_int;

    pub fn nvme_ctrl_deverr(
        arg1: *mut nvme_ctrl_t,
        arg2: *mut u32,
        arg3: *mut u32,
    );

    // NVMe Controller discovery.
    pub fn nvme_ctrl_disc_devi(arg1: *const nvme_ctrl_disc_t) -> *mut di_node;
    pub fn nvme_ctrl_discover_init(
        arg1: *mut nvme_t,
        arg2: *mut *mut nvme_ctrl_iter_t,
    ) -> bool;
    pub fn nvme_ctrl_discover_step(
        arg1: *mut nvme_ctrl_iter_t,
        arg2: *mut *const nvme_ctrl_disc_t,
    ) -> nvme_iter_t;
    pub fn nvme_ctrl_discover_fini(arg1: *mut nvme_ctrl_iter_t);
    pub fn nvme_ctrl_init(
        arg1: *mut nvme_t,
        arg2: *mut di_node,
        arg3: *mut *mut nvme_ctrl_t,
    ) -> bool;
    pub fn nvme_ctrl_init_by_instance(
        arg1: *mut nvme_t,
        arg2: i32,
        arg3: *mut *mut nvme_ctrl_t,
    ) -> bool;
    pub fn nvme_ctrl_fini(arg1: *mut nvme_ctrl_t);

    // NVMe Controller information. Information about a controller is a
    // separate lifetime than the controller itself.
    pub fn nvme_ctrl_info_snap(
        arg1: *mut nvme_ctrl_t,
        arg2: *mut *mut nvme_ctrl_info_t,
    ) -> bool;
    pub fn nvme_ctrl_info_free(arg1: *mut nvme_ctrl_info_t);

    // NVMe Controller information errors.
    pub fn nvme_ctrl_info_err(arg1: *mut nvme_ctrl_info_t) -> nvme_info_err_t;
    pub fn nvme_ctrl_info_errmsg(arg1: *mut nvme_ctrl_info_t) -> *const c_char;
    pub fn nvme_ctrl_info_syserr(arg1: *mut nvme_ctrl_info_t) -> c_int;

    // Information aobut an NVMe Controller
    pub fn nvme_ctrl_info_identify(
        arg1: *mut nvme_ctrl_info_t,
    ) -> *const nvme_identify_ctrl_t;
    pub fn nvme_ctrl_info_model(arg1: *mut nvme_ctrl_info_t) -> *const c_char;
    pub fn nvme_ctrl_info_serial(arg1: *mut nvme_ctrl_info_t) -> *const c_char;
    pub fn nvme_ctrl_info_fwrev(arg1: *mut nvme_ctrl_info_t) -> *const c_char;
    pub fn nvme_ctrl_info_nns(arg1: *mut nvme_ctrl_info_t) -> u32;
    pub fn nvme_ctrl_info_pci_vid(
        arg1: *mut nvme_ctrl_info_t,
        arg2: *mut u16,
    ) -> bool;

    // NVM command set for controllers.
    pub fn nvme_ctrl_info_nformats(arg1: *mut nvme_ctrl_info_t) -> u32;
    pub fn nvme_ctrl_info_format(
        arg1: *mut nvme_ctrl_info_t,
        arg2: u32,
        arg3: *mut *const nvme_nvm_lba_fmt_t,
    ) -> bool;
    pub fn nvme_nvm_lba_fmt_id(arg1: *const nvme_nvm_lba_fmt_t) -> u32;
    pub fn nvme_nvm_lba_fmt_meta_size(arg1: *const nvme_nvm_lba_fmt_t) -> u32;
    pub fn nvme_nvm_lba_fmt_data_size(arg1: *const nvme_nvm_lba_fmt_t) -> u64;
    pub fn nvme_nvm_lba_fmt_rel_perf(arg1: *const nvme_nvm_lba_fmt_t) -> u32;

    // NVMe Namespace Discovery.
    pub fn nvme_ns_discover_init(
        arg1: *mut nvme_ctrl_t,
        arg2: nvme_ns_disc_level_t,
        arg2: *mut *mut nvme_ns_iter_t,
    ) -> bool;
    pub fn nvme_ns_discover_fini(arg1: *mut nvme_ns_iter_t);
    pub fn nvme_ns_discover_step(
        arg1: *mut nvme_ns_iter_t,
        arg2: *mut *const nvme_ns_disc_t,
    ) -> nvme_iter_t;
    pub fn nvme_ns_disc_nsid(arg1: *const nvme_ns_disc_t) -> u32;
    pub fn nvme_ns_init(
        arg1: *mut nvme_ctrl_t,
        arg2: u32,
        arg2: *mut *mut nvme_ns_t,
    ) -> bool;
    pub fn nvme_ns_fini(arg1: *mut nvme_ns_t);

    // NVMe Namespace information.
    pub fn nvme_ns_info_snap(
        arg1: *mut nvme_ns_t,
        arg2: *mut *mut nvme_ns_info_t,
    ) -> bool;
    pub fn nvme_ns_info_free(arg1: *mut nvme_ns_info_t);
    pub fn nvme_ns_info_err(arg1: *mut nvme_ns_info_t) -> nvme_info_err_t;
    pub fn nvme_ns_info_errmsg(arg1: *mut nvme_ns_info_t) -> *const c_char;
    pub fn nvme_ns_info_syserr(arg1: *mut nvme_ns_info_t) -> c_int;
    pub fn nvme_ns_info_curformat(
        arg1: *mut nvme_ns_info_t,
        arg2: *mut *const nvme_nvm_lba_fmt,
    ) -> bool;

    // Controller Locking.
    pub fn nvme_ctrl_lock(
        arg1: *mut nvme_ctrl_t,
        arg2: nvme_lock_level_t,
        arg3: nvme_lock_flags_t,
    ) -> bool;
    pub fn nvme_ctrl_unlock(arg1: *mut nvme_ctrl_t);

    // Namespace Attach and Detach.
    pub fn nvme_ns_bd_attach(arg1: *mut nvme_ns_t) -> bool;
    pub fn nvme_ns_bd_detach(arg1: *mut nvme_ns_t) -> bool;

    // NVMe Log Page Discovery
    pub fn nvme_log_req_init_by_name(
        arg1: *mut nvme_ctrl_t,
        arg2: *const c_char,
        arg3: c_uint,
        arg4: *mut *mut nvme_log_disc_t,
        arg5: *mut *mut nvme_log_req_t,
    ) -> bool;
    pub fn nvme_log_disc_size(
        arg1: *const nvme_log_disc_t,
        arg2: *mut u64,
    ) -> nvme_log_size_kind_t;
    pub fn nvme_log_req_set_output(
        arg1: *mut nvme_log_req_t,
        arg2: *mut c_void,
        arg3: usize,
    ) -> bool;
    pub fn nvme_log_disc_calc_size(
        arg1: *const nvme_log_disc_t,
        arg2: *mut u64,
        arg3: *const c_void,
        arg4: usize,
    ) -> bool;
    pub fn nvme_log_req_exec(arg1: *mut nvme_log_req_t) -> bool;
    pub fn nvme_log_disc_free(arg1: *mut nvme_log_disc_t);
    pub fn nvme_log_req_fini(arg1: *mut nvme_log_req_t);

    // Firmware Download and Commit (Activation)
    pub fn nvme_fw_load(
        arg1: *mut nvme_ctrl_t,
        arg2: *const c_void,
        arg3: usize,
        arg4: u64,
    ) -> bool;
    pub fn nvme_fw_commit_req_init(
        arg1: *mut nvme_ctrl_t,
        arg2: *mut *mut nvme_fw_commit_req_t,
    ) -> bool;
    pub fn nvme_fw_commit_req_fini(arg1: *mut nvme_fw_commit_req_t);
    pub fn nvme_fw_commit_req_set_slot(
        arg1: *mut nvme_fw_commit_req_t,
        arg2: u32,
    ) -> bool;
    pub fn nvme_fw_commit_req_set_action(
        arg1: *mut nvme_fw_commit_req_t,
        arg2: u32,
    ) -> bool;
    pub fn nvme_fw_commit_req_exec(arg1: *mut nvme_fw_commit_req_t) -> bool;

    // Format NVM
    //
    // These are used to erase and reformat either all namespaces or a specific
    // one.
    pub fn nvme_format_req_init(
        arg1: *mut nvme_ctrl_t,
        arg2: *mut *mut nvme_format_req_t,
    ) -> bool;
    pub fn nvme_format_req_set_lbaf(
        arg1: *mut nvme_format_req_t,
        arg2: u32,
    ) -> bool;
    pub fn nvme_format_req_set_ses(
        arg1: *mut nvme_format_req_t,
        arg2: u32,
    ) -> bool;
    pub fn nvme_format_req_set_nsid(
        arg1: *mut nvme_format_req_t,
        arg2: u32,
    ) -> bool;
    pub fn nvme_format_req_exec(arg1: *mut nvme_format_req_t) -> bool;
    pub fn nvme_format_req_fini(arg1: *mut nvme_format_req_t);

    // WDC resizing functions.  These are interfaces supported in the SN840,
    // SN650, SN655, etc.
    pub fn nvme_wdc_resize_set(arg1: *mut nvme_ctrl_t, arg2: u32) -> bool;
    pub fn nvme_wdc_resize_get(arg1: *mut nvme_ctrl_t, arg2: *mut u32) -> bool;

}
