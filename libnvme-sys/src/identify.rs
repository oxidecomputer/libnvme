use std::ffi::c_char;

use bitfield_struct::bitfield;
// use static_assertions as sa;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct nvme_uint128_t {
    pub lo: u64,
    pub hi: u64,
}

/// NVMe Queue Entry Size bitfield
#[bitfield(u8)]
pub struct nvme_idctl_qes_t {
    /// minimum entry size
    #[bits(4, access = RO)]
    pub qes_min: u8,
    /// maximum entry size
    #[bits(4, access = RO)]
    pub qes_max: u8,
}

/// NVMe Power State Descriptor/
#[bitfield(u64)]
pub struct nvme_idctl_psd_t_chunk_1 {
    /// Maximum Power
    #[bits(16, access = RO)]
    pub psd_mp: u16,
    /// psd_rsvd1
    #[bits(8)]
    __: B8,
    /// Max Power Scale (1.1)
    #[bits(1, access = RO)]
    pub psd_mps: u8,
    /// Non-Operational State (1.1)
    #[bits(1, access = RO)]
    pub psd_nops: u8,
    /// psd_rsvd2
    #[bits(6)]
    __: B6,
    /// Entry Latency
    #[bits(32, access = RO)]
    pub psd_enlat: u32,
}

#[bitfield(u64)]
pub struct nvme_idctl_psd_t_chunk_2 {
    #[bits(32, access = RO)]
    /* Exit Latency */
    pub psd_exlat: u32,
    #[bits(5, access = RO)]
    /* Relative Read Throughput */
    pub psd_rrt: u8,
    /// psd_rsvd3
    #[bits(3)]
    __: B3,
    #[bits(5, access = RO)]
    /* Relative Read Latency */
    pub psd_rrl: u8,
    /// psd_rsvd4
    #[bits(3)]
    __: B3,
    #[bits(5, access = RO)]
    /// Relative Write Throughput
    pub psd_rwt: u8,
    /// psd_rsvd5
    #[bits(3)]
    __: B3,
    #[bits(5, access = RO)]
    /// Relative Write Latency
    pub psd_rwl: u8,
    /// psd_rsvd6
    #[bits(3)]
    __: B3,
}

#[bitfield(u64)]
pub struct nvme_idctl_psd_t_chunk_3 {
    /// Idle Power (1.2)
    #[bits(16, access = RO)]
    pub psd_idlp: u16,
    /// psd_rsvd7
    #[bits(6)]
    __: u8,
    /// Idle Power Scale (1.2)
    #[bits(2, access = RO)]
    pub psd_ips: u8,
    /// psd_rsvd8
    #[bits(8)]
    __: B8,
    /// Active Power (1.2)
    #[bits(16)]
    pub psd_actp: u16,
    /// Active Power Workload (1.2)
    #[bits(3, access = RO)]
    pub psd_apw: u8,
    /// psd_rsvd9
    #[bits(3)]
    __: B3,
    /// Active Power Scale
    #[bits(2, access = RO)]
    pub psd_aps: u8,
    /// psd_rsvd10 -- the last 64 bits are in the top level struct
    #[bits(8)]
    __: B8,
}

// This struct is quite large and `bitfield-struct` requires us to map to a
// primative type. To work around this we break the top level struct up into
// chunks of 64bits so that we can avoid any possible alignment issues with
// u128.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct nvme_idctl_psd_t {
    pub chunk1: nvme_idctl_psd_t_chunk_1,
    pub chunk2: nvme_idctl_psd_t_chunk_2,
    pub chunk3: nvme_idctl_psd_t_chunk_3,
    _padding: u64,
}

const NVME_SERIAL_SZ: usize = 20;
const NVME_MODEL_SZ: usize = 40;
const NVME_FWVER_SZ: usize = 8;

#[bitfield(u8)]
/// Multi-Interface Capabilities
pub struct IdMic {
    /// HW has multiple PCIe interfaces
    #[bits(1, access = RO)]
    pub m_multi_pci: u8,
    /// HW has multiple controllers (1.1)
    #[bits(1, access = RO)]
    pub m_multi_ctrl: u8,
    /// Controller is SR-IOV virt fn (1.1)
    #[bits(1, access = RO)]
    pub m_sr_iov: u8,
    /// ANA Reporting Supported (1.4)
    #[bits(1, access = RO)]
    pub m_anar_sup: u8,
    /// m_rsvd
    #[bits(4)]
    __: B4,
}

#[bitfield(u32)]
pub struct IdOaes {
    /// oaes_rsvd0
    #[bits(8)]
    __: B8,
    /// Namespace Attribute Notices (1.2)
    #[bits(1, access = RO)]
    pub oaes_nsan: u8,
    /// Firmware Activation Notices (1.2)
    #[bits(1, access = RO)]
    #[bits(1, access = RO)]
    pub oaes_fwact: u8,
    /// oaes_rsvd1
    #[bits(1)]
    pub oaes_rsvd1: u8,
    /// Asymmetric NS Access Change (1.4)
    #[bits(1, access = RO)]
    pub oaes_ansacn: u8,
    /// Predictable Lat Event Agg. (1.4)
    #[bits(1, access = RO)]
    pub oaes_plat: u8,
    /// LBA Status Information (1.4)
    #[bits(1, access = RO)]
    pub oaes_lbasi: u8,
    /// Endurance Group Event Agg. (1.4)
    #[bits(1, access = RO)]
    pub oaes_egeal: u8,
    /// oaes_rsvd2
    #[bits(17)]
    __: B17,
}

#[bitfield(u32)]
pub struct IdCtratt {
    /// 128-bit Host Identifier (1.2)
    #[bits(1, access = RO)]
    pub ctrat_hid: u8,
    /// Non-Operational Power State (1.3)
    #[bits(1, access = RO)]
    pub ctrat_nops: u8,
    /// NVMe Sets (1.4)
    #[bits(1, access = RO)]
    pub ctrat_nvmset: u8,
    /// Read Recovery Levels (1.4)
    #[bits(1, access = RO)]
    pub ctrat_rrl: u8,
    /// Endurance Groups (1.4)
    #[bits(1, access = RO)]
    pub ctrat_engrp: u8,
    /// Predictable Latency Mode (1.4)
    #[bits(1, access = RO)]
    pub ctrat_plm: u8,
    /// Traffic Based Keep Alive (1.4)
    #[bits(1, access = RO)]
    pub ctrat_tbkas: u8,
    /// Namespace Granularity (1.4)
    #[bits(1, access = RO)]
    pub ctrat_nsg: u8,
    /// SQ Associations (1.4)
    #[bits(1, access = RO)]
    pub ctrat_sqass: u8,
    /// UUID List (1.4)
    #[bits(1, access = RO)]
    pub ctrat_uuid: u8,
    /// ctrat_rsvd
    #[bits(22)]
    __: B22,
}

#[bitfield(u8)]
/// NVMe Subsystem Report
pub struct IdNvmsr {
    /// NVMe Storage Device
    #[bits(1, access = RO)]
    pub nvmsr_nvmesd: u8,
    /// NVMe Enclosure
    #[bits(1, access = RO)]
    pub nvmsr_nvmee: u8,
    /// nvmsr_rsvd
    #[bits(6)]
    __: B6,
}

#[bitfield(u8)]
/// VPD Write Cycle Information
pub struct IdVpdwc {
    /// Write Cycles Remaining
    #[bits(7, access = RO)]
    pub vwci_crem: u8,
    /// Write Cycles Remaining Valid
    #[bits(1, access = RO)]
    pub vwci_valid: u8,
}

#[bitfield(u8)]
/// Management Endpoint Capabilities
pub struct IdMec {
    /// SMBus Port Management Endpoint
    #[bits(1, access = RO)]
    pub mec_smbusme: u8,
    /// PCIe Port Management Endpoint
    #[bits(1, access = RO)]
    pub mec_pcieme: u8,
    /// mec_rsvd
    #[bits(6)]
    __: B6,
}

#[bitfield(u16)]
/// Optional Admin Command Support
pub struct IdOacs {
    /// Security Send & Receive
    #[bits(1, access = RO)]
    pub oa_security: u8,
    /// Format NVM
    #[bits(1, access = RO)]
    pub oa_format: u8,
    /// Firmware Activate & Download
    #[bits(1, access = RO)]
    pub oa_firmware: u8,
    /// Namespace Management (1.2)
    #[bits(1, access = RO)]
    pub oa_nsmgmt: u8,
    /// Self Test (1.3)
    #[bits(1, access = RO)]
    pub oa_selftest: u8,
    /// Directives (1.3)
    #[bits(1, access = RO)]
    pub oa_direct: u8,
    /// MI-Send/Recv (1.3)
    #[bits(1, access = RO)]
    pub oa_nvmemi: u8,
    /// Virtualization Management (1.3)
    #[bits(1, access = RO)]
    pub oa_virtmgmt: u8,
    /// Doorbell Buffer Config (1.3)
    #[bits(1, access = RO)]
    pub oa_doorbell: u8,
    /// LBA Status (1.4)
    #[bits(1, access = RO)]
    pub oa_lbastat: u8,
    /// oa_rsvd
    #[bits(6)]
    __: B6,
}

#[bitfield(u8)]
/// Firmware Updates
pub struct IdFrmw {
    /// Slot 1 is Read-Only
    #[bits(1, access = RO)]
    pub fw_readonly: bool,
    /// number of firmware slots
    #[bits(3, access = RO)]
    pub fw_nslot: u8,
    /// Activate w/o reset (1.2)
    #[bits(1, access = RO)]
    pub fw_norst: u8,
    /// fw_rsvd
    #[bits(3)]
    __: B3,
}

#[bitfield(u8)]
/// Log Page Attributes
pub struct IdLpa {
    /// SMART/Health information per NS
    #[bits(1, access = RO)]
    pub lp_smart: u8,
    /// Command Effects (1.2)
    #[bits(1, access = RO)]
    pub lp_cmdeff: u8,
    /// Extended Get Log Page (1.2)
    #[bits(1, access = RO)]
    pub lp_extsup: u8,
    /// Telemetry Log Pages (1.3)
    #[bits(1, access = RO)]
    pub lp_telemetry: u8,
    /// Persistent Log Page (1.4)
    #[bits(1, access = RO)]
    pub lp_persist: u8,
    /// lp_rsvd
    #[bits(3)]
    __: B3,
}

#[bitfield(u8)]
/// Admin Vendor Specific Command Conf
pub struct IdAvscc {
    /// use format from spec
    #[bits(1, access = RO)]
    pub av_spec: u8,
    /// av_rsvd
    #[bits(7)]
    __: B7,
}

#[bitfield(u8)]
/// Autonomous Power State Trans (1.1)
pub struct IdApsta {
    /// APST supported (1.1)
    #[bits(1, access = RO)]
    pub ap_sup: u8,
    /// ap_rsvd
    #[bits(7)]
    __: B7,
}

#[bitfield(u32)]
/// Replay Protected Mem. Block (1.2)
pub struct ApRpmbs {
    /// Number of targets
    #[bits(3, access = RO)]
    pub rpmbs_units: u8,
    /// Auth method
    #[bits(3, access = RO)]
    pub rpmbs_auth: u8,
    /// rpmbs_rsvd
    #[bits(10)]
    __: B10,
    /// Total size in 128KB
    #[bits(8, access = RO)]
    pub rpmbs_tot: u8,
    /// Access size in 512B
    #[bits(8, access = RO)]
    pub rpmbs_acc: u8,
}

#[bitfield(u8)]
/// Device Self-test Options
pub struct ApDsto {
    /// Subsystem level self-test (1.3)
    #[bits(1, access = RO)]
    pub dsto_sub: u8,
    /// dstro_rsvd
    #[bits(7)]
    __: B7,
}

#[bitfield(u16)]
/// Host Thermal Management (1.3)
pub struct ApHctma {
    /// Host Controlled (1.3)
    #[bits(1, access = RO)]
    pub hctma_hctm: u8,
    /// hctma_rsvd
    #[bits(15)]
    __: B15,
}

#[bitfield(u32)]
/// Sanitize Caps
pub struct ApSanitize {
    /// Crypto Erase Support (1.3)
    #[bits(1, access = RO)]
    pub san_ces: u8,
    /// Block Erase Support (1.3)
    #[bits(1, access = RO)]
    pub san_bes: u8,
    /// Overwite Support (1.3)
    #[bits(1, access = RO)]
    pub san_ows: u8,
    /// san_rsvd
    #[bits(26)]
    __: B26,
    /// No-deallocate Inhibited (1.4)
    #[bits(1, access = RO)]
    pub san_ndi: u8,
    /// No-Deallocate Modifies Media (1.4)
    #[bits(2, access = RO)]
    pub san_nodmmas: u8,
}

#[bitfield(u8)]
/// Asymmetric Namespace Access Caps
pub struct ApAnacap {
    /// Optimized State (1.4)
    #[bits(1, access = RO)]
    pub anacap_opt: u8,
    /// Un-optimized State (1.4)
    #[bits(1, access = RO)]
    pub anacap_unopt: u8,
    /// Inaccessible State (1.4)
    #[bits(1, access = RO)]
    pub anacap_inacc: u8,
    /// Persistent Loss (1.4)
    #[bits(1, access = RO)]
    pub anacap_ploss: u8,
    /// Change State (1.4 )
    #[bits(1, access = RO)]
    pub anacap_chg: u8,
    /// anacap_rsvd
    #[bits(1)]
    __: B1,
    /// ID Changes with NS Attach (1.4)
    #[bits(1, access = RO)]
    pub anacap_grpns: u8,
    /// Supports Group ID (1.4)
    #[bits(1, access = RO)]
    pub anacap_grpid: u8,
}

#[bitfield(u16)]
/// Optional NVM Command Support
pub struct IdOncs {
    /// Compare
    #[bits(1, access = RO)]
    pub on_compare: u8,
    /// Write Uncorrectable
    #[bits(1, access = RO)]
    pub on_wr_unc: u8,
    /// Dataset Management
    #[bits(1, access = RO)]
    pub on_dset_mgmt: u8,
    /// Write Zeros (1.1)
    #[bits(1, access = RO)]
    pub on_wr_zero: u8,
    /// Save/Select in Get/Set Feat (1.1)
    #[bits(1, access = RO)]
    pub on_save: u8,
    /// Reservations (1.1)
    #[bits(1, access = RO)]
    pub on_reserve: u8,
    /// Timestamp (1.3)
    #[bits(1, access = RO)]
    pub on_ts: u8,
    /// Verify (1.4)
    #[bits(1, access = RO)]
    pub on_verify: u8,
    /// on_rsvd
    #[bits(8)]
    __: B8,
}

#[bitfield(u16)]
/// Fused Operation Support
pub struct IdFuses {
    /// Compare and Write
    #[bits(1, access = RO)]
    pub f_cmp_wr: u8,
    /// f_rsvd
    #[bits(15)]
    __: B15,
}

#[bitfield(u8)]
/// Format NVM Attributes
pub struct IdFna {
    /// Format applies to all NS
    #[bits(1, access = RO)]
    pub fn_format: u8,
    /// Secure Erase applies to all NS
    #[bits(1, access = RO)]
    pub fn_sec_erase: u8,
    /// Cryptographic Erase supported
    #[bits(1, access = RO)]
    pub fn_crypt_erase: u8,
    /// fn_rsvd
    #[bits(5)]
    __: B5,
}

#[bitfield(u8)]
/// Volatile Write Cache
pub struct IdVwc {
    /// Volatile Write Cache present
    #[bits(1, access = RO)]
    pub vwc_present: u8,
    /// Flush with NS ffffffff (1.4)
    #[bits(2, access = RO)]
    pub vwc_nsflush: u8,
    /// vwc_rsvd
    #[bits(5)]
    __: B5,
}

#[bitfield(u8)]
/// NVM Vendor Specific Command Conf
pub struct IdNvscc {
    /// use format from spec
    #[bits(1, access = RO)]
    pub nv_spec: u8,
    /// nv_rsvd
    #[bits(7)]
    __: B7,
}

#[bitfield(u8)]
/// Namespace Write Protection Caps
pub struct IdNwpc {
    /// Base support (1.4)
    #[bits(1, access = RO)]
    pub nwpc_base: u8,
    /// Write prot until power cycle (1.4)
    #[bits(1, access = RO)]
    pub nwpc_wpupc: u8,
    /// Permanent write prot (1.4)
    #[bits(1, access = RO)]
    pub nwpc_permwp: u8,
    /// nwpc_rsvd
    #[bits(5)]
    __: B5,
}

#[bitfield(u32)]
/// SGL Support (1.1)
pub struct IdSgls {
    /// SGL Supported in NVM cmds (1.3)
    #[bits(2, access = RO)]
    pub sgl_sup: u8,
    /// Keyed SGL Support (1.2)
    #[bits(1, access = RO)]
    pub sgl_keyed: u8,
    /// sgl_rsvd1
    #[bits(13)]
    __: B13,
    /// SGL Bit Bucket supported (1.1)
    #[bits(1, access = RO)]
    pub sgl_bucket: u8,
    /// SGL Byte Aligned (1.2)
    #[bits(1, access = RO)]
    pub sgl_balign: u8,
    /// SGL Length Longer than Data (1.2)
    #[bits(1, access = RO)]
    pub sgl_sglgtd: u8,
    /// SGL MPTR w/ SGL (1.2)
    #[bits(1, access = RO)]
    pub sgl_mptr: u8,
    /// SGL Address is offset (1.2)
    #[bits(1, access = RO)]
    pub sgl_offset: u8,
    /// Transport SGL Data Block (1.4)
    #[bits(1, access = RO)]
    pub sgl_tport: u8,
    /// sgl_rsvd2
    #[bits(10)]
    __: B10,
}

/// NVMe Identify Controller Data Structure
#[repr(C)]
#[derive(Debug, Clone)]
pub struct nvme_identify_ctrl {
    /* Controller Capabilities & Features */
    pub id_vid: u16,                         /* PCI vendor ID */
    pub id_ssvid: u16,                       /* PCI subsystem vendor ID */
    pub id_serial: [c_char; NVME_SERIAL_SZ], /* Serial Number */
    pub id_model: [c_char; NVME_MODEL_SZ],   /* Model Number */
    pub id_fwrev: [c_char; NVME_FWVER_SZ],   /* Firmware Revision */
    pub id_rab: u8,                          /* Recommended Arbitration Burst */
    pub id_oui: [u8; 3],                     /* vendor IEEE OUI */
    pub id_mic: IdMic,
    pub id_mdts: u8,    /* Maximum Data Transfer Size */
    pub id_cntlid: u16, /* Unique Controller Identifier (1.1) */
    /* Added in NVMe 1.2 */
    pub id_ver: u32,   /* Version (1.2) */
    pub id_rtd3r: u32, /* RTD3 Resume Latency (1.2) */
    pub id_rtd3e: u32, /* RTD3 Entry Latency (1.2) */
    pub id_oaes: IdOaes,
    pub id_ctratt: IdCtratt,
    pub id_rrls: u16, /* Read Recovery Levels (1.4) */
    pub id_rsvd_cc: [u8; 9],
    pub id_cntrltype: u8,    /* Controller Type (1.4) */
    pub id_frguid: [u8; 16], /* FRU GUID (1.3) */
    pub id_crdt1: u16,       /* Command Retry Delay Time 1 (1.4) */
    pub id_crdt2: u16,       /* Command Retry Delay Time 2 (1.4) */
    pub id_crdt3: u16,       /* Command Retry Delay Time 3 (1.4) */
    pub id_rsvd2_cc: [u8; 106],
    pub id_rsvd_nvmemi: [u8; 13],
    /* NVMe-MI region */
    pub id_nvmsr: IdNvmsr,
    pub id_vpdwc: IdVpdwc,
    pub id_mec: IdMec,

    /* Admin Command Set Attributes */
    pub id_oacs: IdOacs,
    pub id_acl: u8,  /* Abort Command Limit */
    pub id_aerl: u8, /* Asynchronous Event Request Limit */
    pub id_frmw: IdFrmw,
    pub id_lpa: IdLpa,
    pub id_elpe: u8, /* Error Log Page Entries */
    pub id_npss: u8, /* Number of Power States */
    pub id_avscc: IdAvscc,
    pub id_apsta: IdApsta,
    pub ap_wctemp: u16, /* Warning Composite Temp. (1.2) */
    pub ap_cctemp: u16, /* Critical Composite Temp. (1.2) */
    pub ap_mtfa: u16,   /* Maximum Firmware Activation (1.2) */
    pub ap_hmpre: u32,  /* Host Memory Buf Pref Size (1.2) */
    pub ap_hmmin: u32,  /* Host Memory Buf Min Size (1.2) */
    pub ap_tnvmcap: nvme_uint128_t, /* Total NVM Capacity in Bytes (1.2) */
    pub ap_unvmcap: nvme_uint128_t, /* Unallocated NVM Capacity (1.2) */
    pub ap_rpmbs: ApRpmbs,
    /* Added in NVMe 1.3 */
    pub ap_edstt: u16, /* Ext. Device Self-test time (1.3) */
    pub ap_dsto: ApDsto,
    pub ap_fwug: u8, /* Firmware Update Granularity (1.3) */
    pub ap_kas: u16, /* Keep Alive Support (1.2) */
    pub ap_hctma: ApHctma,
    pub ap_mntmt: u16, /* Minimum Thermal Temperature (1.3) */
    pub ap_mxtmt: u16, /* Maximum Thermal Temperature (1.3) */
    pub ap_sanitize: ApSanitize,
    pub ap_hmminds: u32, /* Host Mem Buf Min Desc Entry (1.4) */
    pub ap_hmmaxd: u16,  /* How Mem Max Desc Entries (1.4) */
    pub ap_nsetidmax: u16, /* Max NVMe set identifier (1.4) */
    pub ap_engidmax: u16, /* Max Endurance Group ID (1.4) */
    pub ap_anatt: u8,    /* ANA Transition Time (1.4) */
    pub ap_anacap: ApAnacap,
    pub ap_anagrpmax: u32, /* ANA Group ID Max (1.4) */
    pub ap_nanagrpid: u32, /* Number of ANA Group IDs (1.4) */
    pub ap_pels: u32,      /* Persistent Event Log Size (1.4) */
    pub id_rsvd_ac: [u8; 156],

    /* NVM Command Set Attributes */
    pub id_sqes: nvme_idctl_qes_t, /* Submission Queue Entry Size */
    pub id_cqes: nvme_idctl_qes_t, /* Completion Queue Entry Size */
    pub id_maxcmd: u16,            /* Max Outstanding Commands (1.3) */
    pub id_nn: u32,                /* Number of Namespaces */
    pub id_oncs: IdOncs,
    pub id_fuses: IdFuses,
    pub id_fna: IdFna,
    pub id_vwc: IdVwc,
    pub id_awun: u16,  /* Atomic Write Unit Normal */
    pub id_awupf: u16, /* Atomic Write Unit Power Fail */
    pub id_nvscc: IdNvscc,
    pub id_nwpc: IdNwpc,
    pub id_acwu: u16, /* Atomic Compare & Write Unit (1.1) */
    pub id_rsvd_nc_3: u16,
    pub id_sgls: IdSgls,
    pub id_mnan: u32, /* Maximum Number of Allowed NSes */
    pub id_rsvd_nc_4: [u8; 224usize],

    /* I/O Command Set Attributes */
    pub id_subnqn: [u8; 256], /* Subsystem Qualified Name (1.2.1+) */
    pub id_rsvd_ioc: [u8; 768],
    pub id_nvmof: [u8; 256], /* NVMe over Fabrics */

    /* Power State Descriptors */
    pub id_psd: [nvme_idctl_psd_t; 32],

    /* Vendor Specific */
    pub id_vs: [u8; 1024],
}

// XXX static_assertions breaks ctest2.
// Assert that we match the size of nvme_identify_ctrl_t from libnvme.
// sa::assert_eq_size!([u8; 4096], nvme_identify_ctrl);
