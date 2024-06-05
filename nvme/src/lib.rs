// NVMe completion status code type

/// Generic Command Status
pub const NVME_CQE_SCT_GENERIC: u32 = 0;
/// Command Specific Status
pub const NVME_CQE_SCT_SPECIFIC: u32 = 1;
/// Media and Data Integrity Errors
pub const NVME_CQE_SCT_INTEGRITY: u32 = 2;
/// Path Related Status (1.4)
pub const NVME_CQE_SCT_PATH: u32 = 3;
/// Vendor Specific
pub const NVME_CQE_SCT_VENDOR: u32 = 7;

// NVMe completion status code (command specific)

/// Completion Queue Invalid
pub const NVME_CQE_SC_SPC_INV_CQ: u32 = 0x0;
/// Invalid Queue Identifier
pub const NVME_CQE_SC_SPC_INV_QID: u32 = 0x1;
/// Max Queue Size Exceeded
pub const NVME_CQE_SC_SPC_MAX_QSZ_EXC: u32 = 0x2;
/// Abort Cmd Limit Exceeded
pub const NVME_CQE_SC_SPC_ABRT_CMD_EXC: u32 = 0x3;
/// Async Event Request Limit
pub const NVME_CQE_SC_SPC_ASYNC_EVREQ_EXC: u32 = 0x5;
/// Invalid Firmware Slot
pub const NVME_CQE_SC_SPC_INV_FW_SLOT: u32 = 0x6;
/// Invalid Firmware Image
pub const NVME_CQE_SC_SPC_INV_FW_IMG: u32 = 0x7;
/// Invalid Interrupt Vector
pub const NVME_CQE_SC_SPC_INV_INT_VECT: u32 = 0x8;
/// Invalid Log Page
pub const NVME_CQE_SC_SPC_INV_LOG_PAGE: u32 = 0x9;
/// Invalid Format
pub const NVME_CQE_SC_SPC_INV_FORMAT: u32 = 0xa;
/// FW Application Reset Reqd
pub const NVME_CQE_SC_SPC_FW_RESET: u32 = 0xb;
/// Invalid Queue Deletion
pub const NVME_CQE_SC_SPC_INV_Q_DEL: u32 = 0xc;
/// Feature Id Not Saveable
pub const NVME_CQE_SC_SPC_FEAT_SAVE: u32 = 0xd;
/// Feature Not Changeable
pub const NVME_CQE_SC_SPC_FEAT_CHG: u32 = 0xe;
/// Feature Not Namespace Spec
pub const NVME_CQE_SC_SPC_FEAT_NS_SPEC: u32 = 0xf;

// Added in NVMe 1.2

/// FW Application NSSR Reqd
pub const NVME_CQE_SC_SPC_FW_NSSR: u32 = 0x10;
/// FW Application Next Reqd
pub const NVME_CQE_SC_SPC_FW_NEXT_RESET: u32 = 0x11;
/// FW Application Exceed MTFA
pub const NVME_CQE_SC_SPC_FW_MTFA: u32 = 0x12;
/// FW Application Prohibited
pub const NVME_CQE_SC_SPC_FW_PROHIBITED: u32 = 0x13;
/// Overlapping FW ranges
pub const NVME_CQE_SC_SPC_FW_OVERLAP: u32 = 0x14;
/// NS Insufficient Capacity
pub const NVME_CQE_SC_SPC_NS_INSUF_CAP: u32 = 0x15;
/// NS ID Unavailable
pub const NVME_CQE_SC_SPC_NS_NO_ID: u32 = 0x16;

// 0x17 is reserved

/// NS Already Attached
pub const NVME_CQE_SC_SPC_NS_ATTACHED: u32 = 0x18;
/// NS is private
pub const NVME_CQE_SC_SPC_NS_PRIV: u32 = 0x19;
/// NS Not Attached
pub const NVME_CQE_SC_SPC_NS_NOT_ATTACH: u32 = 0x1a;
/// Thin Provisioning ENOTSUP
pub const NVME_CQE_SC_SPC_THIN_ENOTSUP: u32 = 0x1b;
/// Controller list invalid
pub const NVME_CQE_SC_SPC_INV_CTRL_LIST: u32 = 0x1c;

// Added in NVMe 1.3

/// Self-test in progress
pub const NVME_CQE_SC_SPC_SELF_TESTING: u32 = 0x1d;
/// No Boot Partition Write
pub const NVME_CQE_SC_SPC_NO_BP_WRITE: u32 = 0x1e;
/// Invalid Controller Id
pub const NVME_CQE_SC_SPC_INV_CTRL_ID: u32 = 0x1f;
/// Invalid Sec. Ctrl state
pub const NVME_CQE_SC_SPC_INV_SEC_CTRL: u32 = 0x20;
/// Inv. # Ctrl Resources
pub const NVME_CQE_SC_SPC_INV_CTRL_NRSRC: u32 = 0x21;
/// Inv. Resource ID
pub const NVME_CQE_SC_SPC_INV_RSRC_ID: u32 = 0x22;

// Added in NVMe 1.4

/// Sanitize prohib. w/ pmem
pub const NVME_CQE_SC_SPC_NO_SAN_PMR: u32 = 0x23;
/// Invalid ANA group ID
pub const NVME_CQE_SC_SPC_INV_ANA_GID: u32 = 0x24;
/// ANA Attach Failed
pub const NVME_CQE_SC_SPC_ANA_ATTACH: u32 = 0x25;

// Added in NVMe 2.0

/// Insufficient Capacity
pub const NVME_CQE_SC_SPC_INSUF_CAP: u32 = 0x26;
/// NS Attach Limit Exceeded
pub const NVME_CQE_SC_SPC_NS_ATTACH_LIM: u32 = 0x27;
/// Prohib Cmd Exec Not Sup
pub const NVME_CQE_SC_SPC_LOCKDOWN_UNSUP: u32 = 0x28;
/// I/O Command set not sup
pub const NVME_CQE_SC_SPC_UNSUP_IO_CMD: u32 = 0x29;
/// I/O Command set not enab
pub const NVME_CQE_SC_SPC_DIS_IO_CMD: u32 = 0x2a;
/// I/O command set combo rej
pub const NVME_CQE_SC_SPC_INV_CMD_COMBO: u32 = 0x2b;
/// Invalid I/O command set
pub const NVME_CQE_SC_SPC_INV_IO_CMD: u32 = 0x2c;
/// Unavailable ID
pub const NVME_CQE_SC_SPC_UNAVAIL_ID: u32 = 0x2d;
