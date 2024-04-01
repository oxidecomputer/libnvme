use std::ffi::c_char;

use bitfield_struct::bitfield;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct nvme_uint128_t {
    pub lo: u64,
    pub hi: u64,
}

/* NVMe Queue Entry Size bitfield */
#[bitfield(u8)]
pub struct nvme_idctl_qes_t {
    /// minimum entry size
    #[bits(4, access = RO)]
    qes_min: u8,
    /// maximum entry size
    #[bits(4, access = RO)]
    qes_max: u8,
}

/* NVMe Power State Descriptor */
#[bitfield(u64)]
pub struct nvme_idctl_psd_t_chunk_1 {
    /// Maximum Power
    psd_mp: u16,
    #[bits(8)]
    /// psd_rsvd1
    __: u8,
    #[bits(1, access = RO)]
    /// Max Power Scale (1.1)
    psd_mps: u8,
    #[bits(1, access = RO)]
    /// Non-Operational State (1.1)
    psd_nops: u8,
    #[bits(6)]
    /// psd_rsvd2
    __: B6,
    #[bits(32, access = RO)]
    /// Entry Latency
    psd_enlat: u32,
}

#[bitfield(u64)]
pub struct nvme_idctl_psd_t_chunk_2 {
    #[bits(32, access = RO)]
    /* Exit Latency */
    psd_exlat: u32,
    /* Relative Read Throughput */
    #[bits(5, access = RO)]
    psd_rrt: u8,
    #[bits(3)]
    /// psd_rsvd3
    __: B3,
    #[bits(5, access = RO)]
    /* Relative Read Latency */
    psd_rrl: u8,
    #[bits(3)]
    /// psd_rsvd4
    __: B3,
    #[bits(5, access = RO)]
    /* Relative Write Throughput */
    psd_rwt: u8,
    #[bits(3)]
    /// psd_rsvd5
    __: B3,
    #[bits(5, access = RO)]
    /* Relative Write Latency */
    psd_rwl: u8,
    #[bits(3)]
    /// psd_rsvd6
    __: B3,
}

#[bitfield(u64)]
pub struct nvme_idctl_psd_t_chunk_3 {
    /// Idle Power (1.2)
    psd_idlp: u16,
    #[bits(6, access = RO)]
    psd_rsvd7: u8,
    #[bits(2, access = RO)]
    /// Idle Power Scale (1.2)
    psd_ips: u8,
    #[bits(8)]
    /// psd_rsvd8
    __: B8,
    #[bits(16)]
    /// Active Power (1.2)
    psd_actp: u16,
    #[bits(3, access = RO)]
    /// Active Power Workload (1.2)
    psd_apw: u8,
    #[bits(3)]
    /// psd_rsvd9
    __: B3,
    #[bits(2, access = RO)]
    /// Active Power Scale
    psd_aps: u8,
    #[bits(8)]
    /// psd_rsvd10 -- the last 64 bits are in the top level struct
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
    #[bits(1, access = RO)]
    /// HW has multiple PCIe interfaces
    m_multi_pci: u8,
    #[bits(1, access = RO)]
    /// HW has multiple controllers (1.1)
    m_multi_ctrl: u8,
    #[bits(1, access = RO)]
    /// Controller is SR-IOV virt fn (1.1)
    m_sr_iov: u8,
    #[bits(1, access = RO)]
    /// ANA Reporting Supported (1.4)
    m_anar_sup: u8,
    /// m_rsvd
    #[bits(4)]
    m_rsvd: u8,
}

#[bitfield(u32)]
pub struct IdOaes {
    #[bits(8)]
    /// oaes_rsvd0
    __: u8,
    #[bits(1, access = RO)]
    /// Namespace Attribute Notices (1.2)
    oaes_nsan: u8,
    #[bits(1, access = RO)]
    /// Firmware Activation Notices (1.2)
    #[bits(1, access = RO)]
    oaes_fwact: u8,
    #[bits(1)]
    /// oaes_rsvd1
    oaes_rsvd1: u8,
    #[bits(1, access = RO)]
    /// Asymmetric NS Access Change (1.4)
    oaes_ansacn: u8,
    #[bits(1, access = RO)]
    /// Predictable Lat Event Agg. (1.4)
    oaes_plat: u8,
    #[bits(1, access = RO)]
    /// LBA Status Information (1.4)
    oaes_lbasi: u8,
    #[bits(1, access = RO)]
    /// Endurance Group Event Agg. (1.4)
    oaes_egeal: u8,
    /// oaes_rsvd2
    #[bits(17)]
    __: B17,
}

#[bitfield(u32)]
pub struct IdCtratt {
    #[bits(1, access = RO)]
    /// 128-bit Host Identifier (1.2)
    ctrat_hid: u8,
    #[bits(1, access = RO)]
    /// Non-Operational Power State (1.3)
    ctrat_nops: u8,
    #[bits(1, access = RO)]
    /// NVMe Sets (1.4)
    ctrat_nvmset: u8,
    #[bits(1, access = RO)]
    /// Read Recovery Levels (1.4)
    ctrat_rrl: u8,
    #[bits(1, access = RO)]
    /// Endurance Groups (1.4)
    ctrat_engrp: u8,
    #[bits(1, access = RO)]
    /// Predictable Latency Mode (1.4)
    ctrat_plm: u8,
    #[bits(1, access = RO)]
    /// Traffic Based Keep Alive (1.4)
    ctrat_tbkas: u8,
    #[bits(1, access = RO)]
    /// Namespace Granularity (1.4)
    ctrat_nsg: u8,
    #[bits(1, access = RO)]
    /// SQ Associations (1.4)
    ctrat_sqass: u8,
    #[bits(1, access = RO)]
    /// UUID List (1.4)
    ctrat_uuid: u8,
    #[bits(22)]
    /// ctrat_rsvd
    __: B22,
}

#[bitfield(u8)]
/// NVMe Subsystem Report
pub struct IdNvmsr {
    #[bits(1, access = RO)]
    /// NVMe Storage Device
    nvmsr_nvmesd: u8,
    #[bits(1, access = RO)]
    /// NVMe Enclosure
    nvmsr_nvmee: u8,
    #[bits(6)]
    /// nvmsr_rsvd
    __: u8,
}

#[bitfield(u8)]
/// VPD Write Cycle Information
pub struct IdVpdwc {
    #[bits(7, access = RO)]
    /// Write Cycles Remaining
    vwci_crem: u8,
    #[bits(1, access = RO)]
    /// Write Cycles Remaining Valid
    vwci_valid: u8,
}

#[bitfield(u8)]
/// Management Endpoint Capabilities
pub struct IdMec {
    #[bits(1, access = RO)]
    /// SMBus Port Management Endpoint
    mec_smbusme: u8,
    #[bits(1, access = RO)]
    /// PCIe Port Management Endpoint
    mec_pcieme: u8,
    #[bits(6)]
    /// mec_rsvd
    __: u8,
}

#[bitfield(u16)]
/// Optional Admin Command Support
pub struct IdOacs {
    #[bits(1, access = RO)]
    /// Security Send & Receive
    oa_security: u8,
    #[bits(1, access = RO)]
    /// Format NVM
    oa_format: u8,
    #[bits(1, access = RO)]
    /// Firmware Activate & Download
    oa_firmware: u8,
    #[bits(1, access = RO)]
    /// Namespace Management (1.2)
    oa_nsmgmt: u8,
    #[bits(1, access = RO)]
    /// Self Test (1.3)
    oa_selftest: u8,
    #[bits(1, access = RO)]
    /// Directives (1.3)
    oa_direct: u8,
    #[bits(1, access = RO)]
    /// MI-Send/Recv (1.3)
    oa_nvmemi: u8,
    #[bits(1, access = RO)]
    /// Virtualization Management (1.3)
    oa_virtmgmt: u8,
    #[bits(1, access = RO)]
    /// Doorbell Buffer Config (1.3)
    oa_doorbell: u8,
    #[bits(1, access = RO)]
    /// LBA Status (1.4)
    oa_lbastat: u8,
    #[bits(6)]
    /// oa_rsvd
    __: B6,
}

#[bitfield(u8)]
/// Firmware Updates
pub struct IdFrmw {
    #[bits(1, access = RO)]
    /// Slot 1 is Read-Only
    pub fw_readonly: bool,
    #[bits(3, access = RO)]
    /// number of firmware slots
    pub fw_nslot: u8,
    #[bits(1, access = RO)]
    /// Activate w/o reset (1.2)
    fw_norst: u8,
    #[bits(3)]
    /// fw_rsvd
    __: u8,
}

#[bitfield(u8)]
/// Log Page Attributes
pub struct IdLpa {
    #[bits(1, access = RO)]
    /// SMART/Health information per NS
    lp_smart: u8,
    #[bits(1, access = RO)]
    /// Command Effects (1.2)
    lp_cmdeff: u8,
    #[bits(1, access = RO)]
    /// Extended Get Log Page (1.2)
    lp_extsup: u8,
    #[bits(1, access = RO)]
    /// Telemetry Log Pages (1.3)
    lp_telemetry: u8,
    #[bits(1, access = RO)]
    /// Persistent Log Page (1.4)
    lp_persist: u8,
    #[bits(3)]
    /// lp_rsvd
    __: B3,
}

#[bitfield(u8)]
/// Admin Vendor Specific Command Conf
pub struct IdAvscc {
    #[bits(1, access = RO)]
    /// use format from spec
    av_spec: u8,
    #[bits(7)]
    /// av_rsvd
    __: B7,
}

#[bitfield(u8)]
/// Autonomous Power State Trans (1.1)
pub struct IdApsta {
    #[bits(1, access = RO)]
    /// APST supported (1.1)
    ap_sup: u8,
    #[bits(7)]
    /// ap_rsvd
    __: B7,
}

#[bitfield(u32)]
/// Replay Protected Mem. Block (1.2)
pub struct ApRpmbs {
    #[bits(3, access = RO)]
    /// Number of targets
    rpmbs_units: u8,
    #[bits(3, access = RO)]
    /// Auth method
    rpmbs_auth: u8,
    #[bits(10)]
    /// rpmbs_rsvd
    __: B10,
    #[bits(8, access = RO)]
    /// Total size in 128KB
    rpmbs_tot: u8,
    #[bits(8, access = RO)]
    /// Access size in 512B
    rpmbs_acc: u8,
}

#[bitfield(u8)]
/// Device Self-test Options
pub struct ApDsto {
    #[bits(1, access = RO)]
    /// Subsystem level self-test (1.3)
    dsto_sub: u8,
    #[bits(7)]
    /// dstro_rsvd
    __: B7,
}

#[bitfield(u16)]
/// Host Thermal Management (1.3)
pub struct ApHctma {
    #[bits(1, access = RO)]
    /// Host Controlled (1.3)
    hctma_hctm: u8,
    #[bits(15)]
    /// hctma_rsvd
    __: B15,
}

#[bitfield(u32)]
/// Sanitize Caps
pub struct ApSanitize {
    #[bits(1, access = RO)]
    /// Crypto Erase Support (1.3)
    san_ces: u8,
    #[bits(1, access = RO)]
    /// Block Erase Support (1.3)
    san_bes: u8,
    #[bits(1, access = RO)]
    /// Overwite Support (1.3)
    san_ows: u8,
    #[bits(26)]
    /// san_rsvd
    __: B26,
    #[bits(1, access = RO)]
    /// No-deallocate Inhibited (1.4)
    san_ndi: u8,
    #[bits(2, access = RO)]
    /// No-Deallocate Modifies Media (1.4)
    san_nodmmas: u8,
}

#[bitfield(u8)]
/// Asymmetric Namespace Access Caps
pub struct ApAnacap {
    #[bits(1, access = RO)]
    /// Optimized State (1.4)
    anacap_opt: u8,
    #[bits(1, access = RO)]
    /// Un-optimized State (1.4)
    anacap_unopt: u8,
    #[bits(1, access = RO)]
    /// Inaccessible State (1.4)
    anacap_inacc: u8,
    #[bits(1, access = RO)]
    /// Persistent Loss (1.4)
    anacap_ploss: u8,
    #[bits(1, access = RO)]
    /// Change State (1.4 )
    anacap_chg: u8,
    #[bits(1)]
    /// anacap_rsvd
    __: u8,
    #[bits(1, access = RO)]
    /// ID Changes with NS Attach (1.4)
    anacap_grpns: u8,
    #[bits(1, access = RO)]
    /// Supports Group ID (1.4)
    anacap_grpid: u8,
}

#[bitfield(u16)]
/// Optional NVM Command Support
pub struct IdOncs {
    #[bits(1, access = RO)]
    /// Compare
    on_compare: u8,
    #[bits(1, access = RO)]
    /// Write Uncorrectable
    on_wr_unc: u8,
    #[bits(1, access = RO)]
    /// Dataset Management
    on_dset_mgmt: u8,
    #[bits(1, access = RO)]
    /// Write Zeros (1.1)
    on_wr_zero: u8,
    #[bits(1, access = RO)]
    /// Save/Select in Get/Set Feat (1.1)
    on_save: u8,
    #[bits(1, access = RO)]
    /// Reservations (1.1)
    on_reserve: u8,
    #[bits(1, access = RO)]
    /// Timestamp (1.3)
    on_ts: u8,
    #[bits(1, access = RO)]
    /// Verify (1.4)
    on_verify: u8,
    #[bits(8)]
    /// on_rsvd
    __: u8,
}

#[bitfield(u16)]
/// Fused Operation Support
pub struct IdFuses {
    #[bits(1, access = RO)]
    /// Compare and Write
    f_cmp_wr: u8,
    #[bits(15)]
    /// f_rsvd
    __: B15,
}

#[bitfield(u8)]
/// Format NVM Attributes
pub struct IdFna {
    #[bits(1, access = RO)]
    /// Format applies to all NS
    fn_format: u8,
    #[bits(1, access = RO)]
    /// Secure Erase applies to all NS
    fn_sec_erase: u8,
    #[bits(1, access = RO)]
    /// Cryptographic Erase supported
    fn_crypt_erase: u8,
    #[bits(5)]
    /// fn_rsvd
    __: B5,
}

#[bitfield(u8)]
/// Volatile Write Cache
pub struct IdVwc {
    #[bits(1, access = RO)]
    /// Volatile Write Cache present
    vwc_present: u8,
    #[bits(2, access = RO)]
    /// Flush with NS ffffffff (1.4)
    vwc_nsflush: u8,
    #[bits(5)]
    /// vwc_rsvd
    __: B5,
}

#[bitfield(u8)]
/// NVM Vendor Specific Command Conf
pub struct IdNvscc {
    #[bits(1, access = RO)]
    /// use format from spec
    nv_spec: u8,
    #[bits(7)]
    /// nv_rsvd
    __: B7,
}

#[bitfield(u8)]
/// Namespace Write Protection Caps
pub struct IdNwpc {
    #[bits(1, access = RO)]
    /// Base support (1.4)
    nwpc_base: u8,
    #[bits(1, access = RO)]
    /// Write prot until power cycle (1.4)
    nwpc_wpupc: u8,
    #[bits(1, access = RO)]
    /// Permanent write prot (1.4)
    nwpc_permwp: u8,
    #[bits(5)]
    /// nwpc_rsvd
    __: B5,
}

#[bitfield(u32)]
/// SGL Support (1.1)
pub struct IdSgls {
    #[bits(2, access = RO)]
    /// SGL Supported in NVM cmds (1.3)
    sgl_sup: u8,
    #[bits(1, access = RO)]
    /// Keyed SGL Support (1.2)
    sgl_keyed: u8,
    #[bits(13)]
    /// sgl_rsvd1
    __: B13,
    #[bits(1, access = RO)]
    /// SGL Bit Bucket supported (1.1)
    sgl_bucket: u8,
    #[bits(1, access = RO)]
    /// SGL Byte Aligned (1.2)
    sgl_balign: u8,
    #[bits(1, access = RO)]
    /// SGL Length Longer than Data (1.2)
    sgl_sglgtd: u8,
    #[bits(1, access = RO)]
    /// SGL MPTR w/ SGL (1.2)
    sgl_mptr: u8,
    #[bits(1, access = RO)]
    /// SGL Address is offset (1.2)
    sgl_offset: u8,
    #[bits(1, access = RO)]
    /// Transport SGL Data Block (1.4)
    sgl_tport: u8,
    #[bits(10)]
    /// sgl_rsvd2
    __: B10,
}

/// NVMe Identify Controller Data Structure
#[repr(C)]
#[derive(Debug, Clone)]
pub struct nvme_identify_ctrl_t {
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

#[cfg(test)]
mod tests {
    #[test]
    fn nvme_identify_ctrl_t_size() {
        // $ mdb /usr/lib/amd64/libnvme.so
        // > ::sizeof nvme_identify_ctrl_t
        // sizeof (nvme_identify_ctrl_t) = 0x1000
        assert_eq!(
            0x1000,
            std::mem::size_of::<crate::identify::nvme_identify_ctrl_t>()
        );
    }
}
