// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Decoders for the bitfields of the Identify Controller data structure.
//!
//! `libnvme-sys` exposes each group of bitfields as a single integer. Wrap
//! that integer with `from_bits` to read the individual fields.

// Most of these are not read yet.
#![allow(dead_code)]

use bitfield_struct::bitfield;

/// NVMe Queue Entry Size bitfield
#[bitfield(u8)]
pub(crate) struct IdQes {
    /// minimum entry size
    #[bits(4, access = RO)]
    pub qes_min: u8,
    /// maximum entry size
    #[bits(4, access = RO)]
    pub qes_max: u8,
}

/// NVMe Power State Descriptor/
#[bitfield(u64)]
pub(crate) struct IdPsdChunk1 {
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
pub(crate) struct IdPsdChunk2 {
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
pub(crate) struct IdPsdChunk3 {
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

#[bitfield(u8)]
/// Multi-Interface Capabilities
pub(crate) struct IdMic {
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
pub(crate) struct IdOaes {
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
pub(crate) struct IdCtratt {
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
pub(crate) struct IdNvmsr {
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
pub(crate) struct IdVpdwc {
    /// Write Cycles Remaining
    #[bits(7, access = RO)]
    pub vwci_crem: u8,
    /// Write Cycles Remaining Valid
    #[bits(1, access = RO)]
    pub vwci_valid: u8,
}

#[bitfield(u8)]
/// Management Endpoint Capabilities
pub(crate) struct IdMec {
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
pub(crate) struct IdOacs {
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
pub(crate) struct IdFrmw {
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
pub(crate) struct IdLpa {
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
pub(crate) struct IdAvscc {
    /// use format from spec
    #[bits(1, access = RO)]
    pub av_spec: u8,
    /// av_rsvd
    #[bits(7)]
    __: B7,
}

#[bitfield(u8)]
/// Autonomous Power State Trans (1.1)
pub(crate) struct IdApsta {
    /// APST supported (1.1)
    #[bits(1, access = RO)]
    pub ap_sup: u8,
    /// ap_rsvd
    #[bits(7)]
    __: B7,
}

#[bitfield(u32)]
/// Replay Protected Mem. Block (1.2)
pub(crate) struct ApRpmbs {
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
pub(crate) struct ApDsto {
    /// Subsystem level self-test (1.3)
    #[bits(1, access = RO)]
    pub dsto_sub: u8,
    /// dstro_rsvd
    #[bits(7)]
    __: B7,
}

#[bitfield(u16)]
/// Host Thermal Management (1.3)
pub(crate) struct ApHctma {
    /// Host Controlled (1.3)
    #[bits(1, access = RO)]
    pub hctma_hctm: u8,
    /// hctma_rsvd
    #[bits(15)]
    __: B15,
}

#[bitfield(u32)]
/// Sanitize Caps
pub(crate) struct ApSanitize {
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
pub(crate) struct ApAnacap {
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
pub(crate) struct IdOncs {
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
pub(crate) struct IdFuses {
    /// Compare and Write
    #[bits(1, access = RO)]
    pub f_cmp_wr: u8,
    /// f_rsvd
    #[bits(15)]
    __: B15,
}

#[bitfield(u8)]
/// Format NVM Attributes
pub(crate) struct IdFna {
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
pub(crate) struct IdVwc {
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
pub(crate) struct IdNvscc {
    /// use format from spec
    #[bits(1, access = RO)]
    pub nv_spec: u8,
    /// nv_rsvd
    #[bits(7)]
    __: B7,
}

#[bitfield(u8)]
/// Namespace Write Protection Caps
pub(crate) struct IdNwpc {
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
pub(crate) struct IdSgls {
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
