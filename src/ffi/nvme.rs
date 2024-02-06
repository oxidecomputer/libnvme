// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_char, c_int, c_uint};

use super::{devinfo::di_node_t, opaque_type};

pub(crate) const NVME_ERR_OK: nvme_err_t = 0;
pub(crate) const NVME_ERR_CONTROLLER: nvme_err_t = 1;
pub(crate) const NVME_ERR_NO_MEM: nvme_err_t = 2;
pub(crate) const NVME_ERR_NO_DMA_MEM: nvme_err_t = 3;
pub(crate) const NVME_ERR_LIBDEVINFO: nvme_err_t = 4;
pub(crate) const NVME_ERR_INTERNAL: nvme_err_t = 5;
pub(crate) const NVME_ERR_BAD_PTR: nvme_err_t = 6;
pub(crate) const NVME_ERR_BAD_FLAG: nvme_err_t = 7;
pub(crate) const NVME_ERR_BAD_DEVI: nvme_err_t = 8;
pub(crate) const NVME_ERR_BAD_DEVI_PROP: nvme_err_t = 9;
pub(crate) const NVME_ERR_ILLEGAL_INSTANCE: nvme_err_t = 10;
pub(crate) const NVME_ERR_BAD_CONTROLLER: nvme_err_t = 11;
pub(crate) const NVME_ERR_PRIVS: nvme_err_t = 12;
pub(crate) const NVME_ERR_OPEN_DEV: nvme_err_t = 13;
pub(crate) const NVME_ERR_BAD_RESTORE: nvme_err_t = 14;
pub(crate) const NVME_ERR_NS_RANGE: nvme_err_t = 15;
pub(crate) const NVME_ERR_NS_UNUSE: nvme_err_t = 16;
pub(crate) const NVME_ERR_LOG_CSI_RANGE: nvme_err_t = 17;
pub(crate) const NVME_ERR_LOG_LID_RANGE: nvme_err_t = 18;
pub(crate) const NVME_ERR_LOG_LSP_RANGE: nvme_err_t = 19;
pub(crate) const NVME_ERR_LOG_LSI_RANGE: nvme_err_t = 20;
pub(crate) const NVME_ERR_LOG_RAE_RANGE: nvme_err_t = 21;
pub(crate) const NVME_ERR_LOG_SIZE_RANGE: nvme_err_t = 22;
pub(crate) const NVME_ERR_LOG_OFFSET_RANGE: nvme_err_t = 23;
pub(crate) const NVME_ERR_LOG_CSI_UNSUP: nvme_err_t = 24;
pub(crate) const NVME_ERR_LOG_LSP_UNSUP: nvme_err_t = 25;
pub(crate) const NVME_ERR_LOG_LSI_UNSUP: nvme_err_t = 26;
pub(crate) const NVME_ERR_LOG_RAE_UNSUP: nvme_err_t = 27;
pub(crate) const NVME_ERR_LOG_OFFSET_UNSUP: nvme_err_t = 28;
pub(crate) const NVME_ERR_LOG_LSP_UNUSE: nvme_err_t = 29;
pub(crate) const NVME_ERR_LOG_LSI_UNUSE: nvme_err_t = 30;
pub(crate) const NVME_ERR_LOG_RAE_UNUSE: nvme_err_t = 31;
pub(crate) const NVME_ERR_LOG_SCOPE_MISMATCH: nvme_err_t = 32;
pub(crate) const NVME_ERR_LOG_REQ_MISSING_FIELDS: nvme_err_t = 33;
pub(crate) const NVME_ERR_LOG_NAME_UNKNOWN: nvme_err_t = 34;
pub(crate) const NVME_ERR_LOG_UNSUP_BY_DEV: nvme_err_t = 35;
pub(crate) const NVME_ERR_IDENTIFY_UNKNOWN: nvme_err_t = 36;
pub(crate) const NVME_ERR_IDENTIFY_UNSUP_BY_DEV: nvme_err_t = 37;
pub(crate) const NVME_ERR_IDENTIFY_CTRLID_RANGE: nvme_err_t = 38;
pub(crate) const NVME_ERR_IDENTIFY_OUTPUT_RANGE: nvme_err_t = 39;
pub(crate) const NVME_ERR_IDENTIFY_CTRLID_UNSUP: nvme_err_t = 40;
pub(crate) const NVME_ERR_IDENTIFY_CTRLID_UNUSE: nvme_err_t = 41;
pub(crate) const NVME_ERR_IDENTIFY_REQ_MISSING_FIELDS: nvme_err_t = 42;
pub(crate) const NVME_ERR_VUC_UNSUP_BY_DEV: nvme_err_t = 43;
pub(crate) const NVME_ERR_VUC_TIMEOUT_RANGE: nvme_err_t = 44;
pub(crate) const NVME_ERR_VUC_OPCODE_RANGE: nvme_err_t = 45;
pub(crate) const NVME_ERR_VUC_IMPACT_RANGE: nvme_err_t = 46;
pub(crate) const NVME_ERR_VUC_NDT_RANGE: nvme_err_t = 47;
pub(crate) const NVME_ERR_VUC_CANNOT_RW: nvme_err_t = 48;
pub(crate) const NVME_ERR_VUC_NO_RESULTS: nvme_err_t = 49;
pub(crate) const NVME_ERR_VUC_UNKNOWN: nvme_err_t = 50;
pub(crate) const NVME_ERR_VUC_REQ_MISSING_FIELDS: nvme_err_t = 51;
pub(crate) const NVME_ERR_VU_FUNC_UNSUP_BY_DEV: nvme_err_t = 52;
pub(crate) const NVME_ERR_WDC_E6_OFFSET_RANGE: nvme_err_t = 53;
pub(crate) const NVME_ERR_FW_UNSUP_BY_DEV: nvme_err_t = 54;
pub(crate) const NVME_ERR_KERN_FW_IMPOS: nvme_err_t = 55;
pub(crate) const NVME_ERR_FW_LOAD_LEN_RANGE: nvme_err_t = 56;
pub(crate) const NVME_ERR_FW_LOAD_OFFSET_RANGE: nvme_err_t = 57;
pub(crate) const NVME_ERR_FW_COMMIT_SLOT_RANGE: nvme_err_t = 58;
pub(crate) const NVME_ERR_FW_COMMIT_ACTION_RANGE: nvme_err_t = 59;
pub(crate) const NVME_ERR_FW_COMMIT_REQ_MISSING_FIELDS: nvme_err_t = 60;
pub(crate) const NVME_ERR_FW_SLOT_RO: nvme_err_t = 61;
pub(crate) const NVME_ERR_FORMAT_UNSUP_BY_DEV: nvme_err_t = 62;
pub(crate) const NVME_ERR_CRYPTO_SE_UNSUP_BY_DEV: nvme_err_t = 63;
pub(crate) const NVME_ERR_NS_FORMAT_UNSUP_BY_DEV: nvme_err_t = 64;
pub(crate) const NVME_ERR_KERN_FORMAT_UNSUP: nvme_err_t = 65;
pub(crate) const NVME_ERR_FORMAT_LBAF_RANGE: nvme_err_t = 66;
pub(crate) const NVME_ERR_FORMAT_SES_RANGE: nvme_err_t = 67;
pub(crate) const NVME_ERR_FORMAT_PARAM_UNSUP: nvme_err_t = 68;
pub(crate) const NVME_ERR_FORMAT_REQ_MISSING_FIELDS: nvme_err_t = 69;
pub(crate) const NVME_ERR_WDC_E6_REQ_MISSING_FIELDS: nvme_err_t = 70;
pub(crate) const NVME_ERR_FEAT_NAME_UNKNOWN: nvme_err_t = 71;
pub(crate) const NVME_ERR_FEAT_UNSUP_BY_DEV: nvme_err_t = 72;
pub(crate) const NVME_ERR_FEAT_FID_RANGE: nvme_err_t = 73;
pub(crate) const NVME_ERR_FEAT_SEL_RANGE: nvme_err_t = 74;
pub(crate) const NVME_ERR_FEAT_CDW11_RANGE: nvme_err_t = 75;
pub(crate) const NVME_ERR_FEAT_DATA_RANGE: nvme_err_t = 76;
pub(crate) const NVME_ERR_FEAT_SEL_UNSUP: nvme_err_t = 77;
pub(crate) const NVME_ERR_FEAT_CDW11_UNUSE: nvme_err_t = 78;
pub(crate) const NVME_ERR_FEAT_DATA_UNUSE: nvme_err_t = 79;
pub(crate) const NVME_ERR_FEAT_NO_RESULTS: nvme_err_t = 80;
pub(crate) const NVME_ERR_GET_FEAT_REQ_MISSING_FIELDS: nvme_err_t = 81;
pub(crate) const NVME_ERR_NEED_CTRL_WRLOCK: nvme_err_t = 82;
pub(crate) const NVME_ERR_NEED_NS_WRLOCK: nvme_err_t = 83;
pub(crate) const NVME_ERR_CTRL_LOCKED: nvme_err_t = 84;
pub(crate) const NVME_ERR_NS_LOCKED: nvme_err_t = 85;
pub(crate) const NVME_ERR_LOCK_PROG: nvme_err_t = 86;
pub(crate) const NVME_ERR_LOCK_ORDER: nvme_err_t = 87;
pub(crate) const NVME_ERR_LOCK_WAIT_INTR: nvme_err_t = 88;
pub(crate) const NVME_ERR_LOCK_WOULD_BLOCK: nvme_err_t = 89;
pub(crate) const NVME_ERR_DETACH_KERN: nvme_err_t = 90;
pub(crate) const NVME_ERR_ATTACH_KERN: nvme_err_t = 91;
pub(crate) const NVME_ERR_ATTACH_UNSUP_KERN: nvme_err_t = 92;
pub(crate) const NVME_ERR_NS_BLKDEV_ATTACH: nvme_err_t = 93;
pub(crate) type nvme_err_t = c_uint;

pub(crate) const NVME_INFO_ERR_OK: nvme_info_err_t = 0;
pub(crate) const NVME_INFO_ERR_TRANSPORT: nvme_info_err_t = 1;
pub(crate) const NVME_INFO_ERR_VERSION: nvme_info_err_t = 2;
pub(crate) const NVME_INFO_ERR_MISSING_CAP: nvme_info_err_t = 3;
pub(crate) const NVME_INFO_ERR_BAD_LBA_FMT: nvme_info_err_t = 4;
pub(crate) const NVME_INFO_ERR_PERSIST_NVL: nvme_info_err_t = 5;
pub(crate) const NVME_INFO_ERR_BAD_FMT: nvme_info_err_t = 6;
pub(crate) const NVME_INFO_ERR_BAD_FMT_DATA: nvme_info_err_t = 7;
pub(crate) const NVME_INFO_ERR_NS_INACTIVE: nvme_info_err_t = 8;
pub(crate) const NVME_INFO_ERR_NS_NO_BLKDEV: nvme_info_err_t = 9;
pub(crate) type nvme_info_err_t = c_uint;

pub(crate) const NVME_ITER_VALID: nvme_iter_t = 0;
pub(crate) const NVME_ITER_DONE: nvme_iter_t = 1;
pub(crate) const NVME_ITER_ERROR: nvme_iter_t = 2;
pub(crate) type nvme_iter_t = c_uint;

pub(crate) type nvme_lock_level_t = c_uint;
pub(crate) type nvme_lock_flags_t = c_uint;

pub const NVME_NS_DISC_F_ALL: nvme_ns_disc_level_t = 0;
pub const NVME_NS_DISC_F_ALLOCATED: nvme_ns_disc_level_t = 1;
pub const NVME_NS_DISC_F_ACTIVE: nvme_ns_disc_level_t = 2;
pub const NVME_NS_DISC_F_NOT_IGNORED: nvme_ns_disc_level_t = 3;
pub const NVME_NS_DISC_F_BLKDEV: nvme_ns_disc_level_t = 4;
pub type nvme_ns_disc_level_t = ::std::os::raw::c_uint;

opaque_type!(nvme_t);
opaque_type!(nvme_ctrl_iter_t);
opaque_type!(nvme_ctrl_disc_t);
opaque_type!(nvme_ctrl_t);
opaque_type!(nvme_ctrl_info_t);
opaque_type!(nvme_ns_iter_t);
opaque_type!(nvme_ns_disc_t);
opaque_type!(nvme_ns_t);
opaque_type!(nvme_nvm_lba_fmt_t);
opaque_type!(nvme_format_req_t);

#[link(name = "nvme")]
extern "C" {
    // NVMe handle.
    pub(crate) fn nvme_init() -> *mut nvme_t;
    pub(crate) fn nvme_fini(arg1: *mut nvme_t);

    /// NVMe Handle errors.
    pub(crate) fn nvme_err(arg1: *mut nvme_t) -> nvme_err_t;
    pub(crate) fn nvme_errmsg(arg1: *mut nvme_t) -> *const c_char;
    pub(crate) fn nvme_syserr(arg1: *mut nvme_t) -> c_int;

    // NVMe Controller errors.
    pub(crate) fn nvme_ctrl_err(arg1: *mut nvme_ctrl_t) -> nvme_err_t;
    pub(crate) fn nvme_ctrl_errmsg(arg1: *mut nvme_ctrl_t) -> *const c_char;
    pub(crate) fn nvme_ctrl_syserr(arg1: *mut nvme_ctrl_t) -> c_int;

    // NVMe Controller discovery.
    pub(crate) fn nvme_ctrl_disc_devi(
        arg1: *const nvme_ctrl_disc_t,
    ) -> di_node_t;
    pub(crate) fn nvme_ctrl_discover_init(
        arg1: *mut nvme_t,
        arg2: *mut *mut nvme_ctrl_iter_t,
    ) -> bool;
    pub(crate) fn nvme_ctrl_discover_step(
        arg1: *mut nvme_ctrl_iter_t,
        arg2: *mut *const nvme_ctrl_disc_t,
    ) -> nvme_iter_t;
    pub(crate) fn nvme_ctrl_discover_fini(arg1: *mut nvme_ctrl_iter_t);
    pub(crate) fn nvme_ctrl_init(
        arg1: *mut nvme_t,
        arg2: di_node_t,
        arg3: *mut *mut nvme_ctrl_t,
    ) -> bool;
    pub(crate) fn nvme_ctrl_fini(arg1: *const nvme_ctrl_t);

    // NVMe Controller information. Information about a controller is a
    // separate lifetime than the controller itself.
    pub(crate) fn nvme_ctrl_info_snap(
        arg1: *mut nvme_ctrl_t,
        arg2: *mut *mut nvme_ctrl_info_t,
    ) -> bool;
    pub(crate) fn nvme_ctrl_info_free(arg1: *mut nvme_ctrl_info_t);

    // NVMe Controller information errors.
    pub(crate) fn nvme_ctrl_info_err(
        arg1: *mut nvme_ctrl_info_t,
    ) -> nvme_info_err_t;
    pub(crate) fn nvme_ctrl_info_errmsg(
        arg1: *mut nvme_ctrl_info_t,
    ) -> *const c_char;
    pub(crate) fn nvme_ctrl_info_syserr(arg1: *mut nvme_ctrl_info_t) -> c_int;

    // Information aobut an NVMe Controller
    pub(crate) fn nvme_ctrl_info_model(
        arg1: *mut nvme_ctrl_info_t,
    ) -> *const c_char;
    pub(crate) fn nvme_ctrl_info_serial(
        arg1: *mut nvme_ctrl_info_t,
    ) -> *const c_char;
    pub(crate) fn nvme_ctrl_info_fwrev(
        arg1: *mut nvme_ctrl_info_t,
    ) -> *const c_char;
    pub(crate) fn nvme_ctrl_info_nns(arg1: *mut nvme_ctrl_info_t) -> u32;
    pub(crate) fn nvme_ctrl_info_pci_vid(
        arg1: *mut nvme_ctrl_info_t,
        arg2: *mut u16,
    ) -> bool;

    // NVM command set for controllers.
    pub(crate) fn nvme_ctrl_info_nformats(arg1: *mut nvme_ctrl_info_t) -> u32;
    pub(crate) fn nvme_ctrl_info_format(
        arg1: *mut nvme_ctrl_info_t,
        arg2: u32,
        arg3: *mut *const nvme_nvm_lba_fmt_t,
    ) -> bool;
    pub(crate) fn nvme_nvm_lba_fmt_id(arg1: *const nvme_nvm_lba_fmt_t) -> u32;
    pub(crate) fn nvme_nvm_lba_fmt_meta_size(
        arg1: *const nvme_nvm_lba_fmt_t,
    ) -> u32;
    pub(crate) fn nvme_nvm_lba_fmt_data_size(
        arg1: *const nvme_nvm_lba_fmt_t,
    ) -> u64;
    pub(crate) fn nvme_nvm_lba_fmt_rel_perf(
        arg1: *const nvme_nvm_lba_fmt_t,
    ) -> u32;

    // NVMe Namespace Discovery.
    pub(crate) fn nvme_ns_discover_init(
        arg1: *mut nvme_ctrl_t,
        arg2: nvme_ns_disc_level_t,
        arg2: *mut *mut nvme_ns_iter_t,
    ) -> bool;
    pub(crate) fn nvme_ns_discover_fini(arg1: *mut nvme_ns_iter_t);
    pub(crate) fn nvme_ns_discover_step(
        arg1: *mut nvme_ns_iter_t,
        arg2: *mut *const nvme_ns_disc_t,
    ) -> nvme_iter_t;
    pub(crate) fn nvme_ns_disc_nsid(arg1: *const nvme_ns_disc_t) -> u32;
    pub(crate) fn nvme_ns_init(
        arg1: *mut nvme_ctrl_t,
        arg2: u32,
        arg2: *mut *mut nvme_ns_t,
    ) -> bool;
    pub(crate) fn nvme_ns_fini(arg1: *mut nvme_ns_t);

    // Controller Locking.
    pub(crate) fn nvme_ctrl_lock(
        arg1: *mut nvme_ctrl_t,
        arg2: nvme_lock_level_t,
        arg3: nvme_lock_flags_t,
    ) -> bool;
    pub(crate) fn nvme_ctrl_unlock(arg1: *mut nvme_ctrl_t);

    // Namespace Attach and Detach.
    pub(crate) fn nvme_ns_bd_attach(arg1: *mut nvme_ns_t) -> bool;
    pub(crate) fn nvme_ns_bd_detach(arg1: *mut nvme_ns_t) -> bool;

    // Format NVM
    //
    // These are used to erase and reformat either all namespaces or a specific
    // one.
    pub(crate) fn nvme_format_req_init(
        arg1: *mut nvme_ctrl_t,
        arg2: *mut *mut nvme_format_req_t,
    ) -> bool;
    pub(crate) fn nvme_format_req_set_lbaf(
        arg1: *mut nvme_format_req_t,
        arg2: u32,
    ) -> bool;
    pub(crate) fn nvme_format_req_set_ses(
        arg1: *mut nvme_format_req_t,
        arg2: u32,
    ) -> bool;
    pub(crate) fn nvme_format_req_set_nsid(
        arg1: *mut nvme_format_req_t,
        arg2: u32,
    ) -> bool;
    pub(crate) fn nvme_format_req_exec(arg1: *mut nvme_format_req_t) -> bool;
    pub(crate) fn nvme_format_req_fini(arg1: *mut nvme_format_req_t);

    // WDC resizing functions.  These are interfaces supported in the SN840,
    // SN650, SN655, etc.
    pub(crate) fn nvme_wdc_resize_set(
        arg1: *mut nvme_ctrl_t,
        arg2: u32,
    ) -> bool;
    pub(crate) fn nvme_wdc_resize_get(
        arg1: *mut nvme_ctrl_t,
        arg2: *mut u32,
    ) -> bool;

}
