// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_char;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct nvme_uint128_t {
    pub lo: u64,
    pub hi: u64,
}

/// NVMe Power State Descriptor
///
/// A `u8` that holds bitfields in C is named after them, with a `_bits`
/// suffix.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct nvme_idctl_psd_t {
    pub psd_mp: u16, /* Maximum Power */
    pub psd_rsvd16: u8,
    pub psd_mps_nops_bits: u8, /* Max Power Scale, Non-Operational State (1.1) */
    pub psd_enlat: u32,        /* Entry Latency */
    pub psd_exlat: u32,        /* Exit Latency */
    pub psd_rrt_bits: u8,      /* Relative Read Throughput */
    pub psd_rrl_bits: u8,      /* Relative Read Latency */
    pub psd_rwt_bits: u8,      /* Relative Write Throughput */
    pub psd_rwl_bits: u8,      /* Relative Write Latency */
    pub psd_idlp: u16,         /* Idle Power (1.2) */
    pub psd_ips_bits: u8,      /* Idle Power Scale (1.2) */
    pub psd_rsvd152: u8,
    pub psd_actp: u16,             /* Active Power (1.2) */
    pub psd_apw_aps_bits: u8, /* Active Power Workload (1.2), Active Power Scale */
    pub psd_epfrt: u8,        /* Emergency Power Fail Recovery Time (2.1) */
    pub psd_fqvt: u8,         /* Forced Quiescence Vault Time (2.1) */
    pub psd_epfvt: u8,        /* Emergency Power Fail Vault Time (2.1) */
    pub psd_epfrts_fqvts_bits: u8, /* EPFRT Scale, FQVT Scale (2.1) */
    pub psd_epfvts_bits: u8,  /* EPFVT Scale (2.1) */
    pub psd_mbw: u8,          /* Max Bandwidth (2.3) */
    pub psd_mbws_bits: u8,    /* Max Bandwidth Scale (2.3) */
    pub psd_rsvd240: [u8; 2],
}

const NVME_SERIAL_SZ: usize = 20;
const NVME_MODEL_SZ: usize = 40;
const NVME_FWVER_SZ: usize = 8;

/// NVMe Identify Controller Data Structure
#[repr(C, packed)]
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
    pub id_mic: u8,                          /* Multi-Interface Capabilities */
    pub id_mdts: u8,                         /* Maximum Data Transfer Size */
    pub id_cntlid: u16, /* Unique Controller Identifier (1.1) */
    /* Added in NVMe 1.2 */
    pub id_ver: u32,    /* Version (1.2) */
    pub id_rtd3r: u32,  /* RTD3 Resume Latency (1.2) */
    pub id_rtd3e: u32,  /* RTD3 Entry Latency (1.2) */
    pub id_oaes: u32,   /* Optional Async Events Supported (1.2) */
    pub id_ctratt: u32, /* Controller Attributes (1.2) */
    pub id_rrls: u16,   /* Read Recovery Levels (1.4) */
    pub id_rsvd_cc: [u8; 9],
    pub id_cntrltype: u8,    /* Controller Type (1.4) */
    pub id_frguid: [u8; 16], /* FRU GUID (1.3) */
    pub id_crdt1: u16,       /* Command Retry Delay Time 1 (1.4) */
    pub id_crdt2: u16,       /* Command Retry Delay Time 2 (1.4) */
    pub id_crdt3: u16,       /* Command Retry Delay Time 3 (1.4) */
    pub id_rsvd2_cc: [u8; 106],
    pub id_rsvd_nvmemi: [u8; 13],
    /* NVMe-MI region */
    pub id_nvmsr: u8, /* NVMe Subsystem Report */
    pub id_vpdwc: u8, /* VPD Write Cycle Information */
    pub id_mec: u8,   /* Management Endpoint Capabilities */

    /* Admin Command Set Attributes */
    pub id_oacs: u16,   /* Optional Admin Command Support */
    pub id_acl: u8,     /* Abort Command Limit */
    pub id_aerl: u8,    /* Asynchronous Event Request Limit */
    pub id_frmw: u8,    /* Firmware Updates */
    pub id_lpa: u8,     /* Log Page Attributes */
    pub id_elpe: u8,    /* Error Log Page Entries */
    pub id_npss: u8,    /* Number of Power States */
    pub id_avscc: u8,   /* Admin Vendor Specific Command Conf */
    pub id_apsta: u8,   /* Autonomous Power State Trans (1.1) */
    pub ap_wctemp: u16, /* Warning Composite Temp. (1.2) */
    pub ap_cctemp: u16, /* Critical Composite Temp. (1.2) */
    pub ap_mtfa: u16,   /* Maximum Firmware Activation (1.2) */
    pub ap_hmpre: u32,  /* Host Memory Buf Pref Size (1.2) */
    pub ap_hmmin: u32,  /* Host Memory Buf Min Size (1.2) */
    pub ap_tnvmcap: nvme_uint128_t, /* Total NVM Capacity in Bytes (1.2) */
    pub ap_unvmcap: nvme_uint128_t, /* Unallocated NVM Capacity (1.2) */
    pub ap_rpmbs: u32,  /* Replay Protected Mem. Block (1.2) */
    /* Added in NVMe 1.3 */
    pub ap_edstt: u16,    /* Ext. Device Self-test time (1.3) */
    pub ap_dsto: u8,      /* Device Self-test Options */
    pub ap_fwug: u8,      /* Firmware Update Granularity (1.3) */
    pub ap_kas: u16,      /* Keep Alive Support (1.2) */
    pub ap_hctma: u16,    /* Host Thermal Management (1.3) */
    pub ap_mntmt: u16,    /* Minimum Thermal Temperature (1.3) */
    pub ap_mxtmt: u16,    /* Maximum Thermal Temperature (1.3) */
    pub ap_sanitize: u32, /* Sanitize Caps */
    pub ap_hmminds: u32,  /* Host Mem Buf Min Desc Entry (1.4) */
    pub ap_hmmaxd: u16,   /* How Mem Max Desc Entries (1.4) */
    pub ap_nsetidmax: u16, /* Max NVMe set identifier (1.4) */
    pub ap_engidmax: u16, /* Max Endurance Group ID (1.4) */
    pub ap_anatt: u8,     /* ANA Transition Time (1.4) */
    pub ap_anacap: u8,    /* Asymmetric Namespace Access Caps */
    pub ap_anagrpmax: u32, /* ANA Group ID Max (1.4) */
    pub ap_nanagrpid: u32, /* Number of ANA Group IDs (1.4) */
    pub ap_pels: u32,     /* Persistent Event Log Size (1.4) */
    pub id_rsvd_ac: [u8; 156],

    /* NVM Command Set Attributes */
    pub id_sqes: u8,    /* Submission Queue Entry Size */
    pub id_cqes: u8,    /* Completion Queue Entry Size */
    pub id_maxcmd: u16, /* Max Outstanding Commands (1.3) */
    pub id_nn: u32,     /* Number of Namespaces */
    pub id_oncs: u16,   /* Optional NVM Command Support */
    pub id_fuses: u16,  /* Fused Operation Support */
    pub id_fna: u8,     /* Format NVM Attributes */
    pub id_vwc: u8,     /* Volatile Write Cache */
    pub id_awun: u16,   /* Atomic Write Unit Normal */
    pub id_awupf: u16,  /* Atomic Write Unit Power Fail */
    pub id_nvscc: u8,   /* NVM Vendor Specific Command Conf */
    pub id_nwpc: u8,    /* Namespace Write Protection Caps */
    pub id_acwu: u16,   /* Atomic Compare & Write Unit (1.1) */
    pub id_rsvd_nc_3: u16,
    pub id_sgls: u32, /* SGL Support (1.1) */
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
    use std::mem::offset_of;

    use super::nvme_idctl_psd_t;

    // header-check can't see these, as C can't take the offset of a bitfield.
    #[test]
    fn nvme_idctl_psd_t_bitfield_offsets() {
        assert_eq!(offset_of!(nvme_idctl_psd_t, psd_mps_nops_bits), 3);
        assert_eq!(offset_of!(nvme_idctl_psd_t, psd_rrt_bits), 12);
        assert_eq!(offset_of!(nvme_idctl_psd_t, psd_rrl_bits), 13);
        assert_eq!(offset_of!(nvme_idctl_psd_t, psd_rwt_bits), 14);
        assert_eq!(offset_of!(nvme_idctl_psd_t, psd_rwl_bits), 15);
        assert_eq!(offset_of!(nvme_idctl_psd_t, psd_ips_bits), 18);
        assert_eq!(offset_of!(nvme_idctl_psd_t, psd_apw_aps_bits), 22);
        assert_eq!(offset_of!(nvme_idctl_psd_t, psd_epfrts_fqvts_bits), 26);
        assert_eq!(offset_of!(nvme_idctl_psd_t, psd_epfvts_bits), 27);
        assert_eq!(offset_of!(nvme_idctl_psd_t, psd_mbws_bits), 29);
    }

    #[test]
    fn nvme_identify_ctrl_t_size() {
        // $ mdb /usr/lib/amd64/libnvme.so
        // > ::sizeof nvme_identify_ctrl_t
        // sizeof (nvme_identify_ctrl_t) = 0x1000
        assert_eq!(
            0x1000,
            std::mem::size_of::<crate::identify::nvme_identify_ctrl>()
        );
    }
}
