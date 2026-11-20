// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The format of the WDC Device Manageability log page, which WDC devices
//! share (`wdc_vsd_t` and `wdc_cbs_t` in illumos `sys/nvme/wdc.h`): a
//! sequence of entries, each a little-endian `u32` length that includes the
//! 8-byte entry header, a `u32` identifier, and data. The first entry is the
//! log header: its length is that of the whole log and its identifier is the
//! log version. What an identifier means is up to the device.

/// Size of an entry header (and of the log header).
const HDR_LEN: usize = 8;

/// Where and why a log is malformed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Malformed {
    pub offset: usize,
    pub reason: &'static str,
}

#[derive(Debug)]
pub(super) struct Entry<'a> {
    /// Offset of the entry's header in the log.
    pub offset: usize,
    pub id: u32,
    pub data: &'a [u8],
}

/// The entries of a log, after its header. Ends after a malformed entry.
pub(super) struct Entries<'a> {
    /// The log, without any data read past its length.
    log: &'a [u8],
    off: usize,
}

/// Returns a log's version and its entries.
pub(super) fn entries(buf: &[u8]) -> Result<(u32, Entries<'_>), Malformed> {
    let malformed = |reason| Malformed { offset: 0, reason };

    let (Some(total), Some(version)) = (u32_at(buf, 0), u32_at(buf, 4)) else {
        return Err(malformed("truncated log header"));
    };
    let end = usize::try_from(total)
        .map_err(|_| malformed("log length too large"))?;
    if end < HDR_LEN {
        return Err(malformed("log length shorter than its header"));
    }
    let Some(log) = buf.get(..end) else {
        return Err(malformed("log length exceeds the data read"));
    };

    Ok((version, Entries { log, off: HDR_LEN }))
}

impl<'a> Iterator for Entries<'a> {
    type Item = Result<Entry<'a>, Malformed>;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.off;
        let end = self.log.len();
        if offset >= end {
            return None;
        }
        self.off = end;
        let malformed = |reason| Some(Err(Malformed { offset, reason }));

        let (Some(len), Some(id)) =
            (u32_at(self.log, offset), u32_at(self.log, offset + 4))
        else {
            return malformed("truncated entry header");
        };
        let Ok(len) = usize::try_from(len) else {
            return malformed("entry length too large");
        };
        if len < HDR_LEN {
            return malformed("entry shorter than its header");
        }
        let Some(next) = offset.checked_add(len).filter(|&next| next <= end)
        else {
            return malformed("entry extends past the log");
        };

        self.off = next;
        Some(Ok(Entry { offset, id, data: &self.log[offset + HDR_LEN..next] }))
    }
}

fn u32_at(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

/// The contents of a counted byte string (`wdc_cbs_t`): a `u32` length
/// followed by that many bytes and any padding.
pub(super) fn cbs(data: &[u8]) -> Option<&[u8]> {
    let len = usize::try_from(u32_at(data, 0)?).ok()?;
    data.get(4..4usize.checked_add(len)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log(words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    #[test]
    fn walks_entries() {
        // A version 3 log with a 4-byte entry and an empty one, then data
        // past the log's length.
        let buf = log(&[28, 3, 12, 0x10, 7, 8, 0x20, 0xffff_ffff]);
        let (version, entries) = entries(&buf).unwrap();
        assert_eq!(version, 3);
        let entries: Vec<_> = entries
            .map(|e| e.unwrap())
            .map(|e| (e.offset, e.id, e.data.to_vec()))
            .collect();
        assert_eq!(
            entries,
            [(8, 0x10, 7u32.to_le_bytes().to_vec()), (20, 0x20, vec![])]
        );
    }

    #[test]
    fn ends_after_a_malformed_entry() {
        // The second entry's length runs past the log.
        let buf = log(&[28, 0, 12, 0x10, 7, 12, 0x20]);
        let (_, mut entries) = entries(&buf).unwrap();
        assert!(entries.next().unwrap().is_ok());
        assert_eq!(
            entries.next().unwrap().err(),
            Some(Malformed {
                offset: 20,
                reason: "entry extends past the log"
            })
        );
        assert!(entries.next().is_none());
    }
}
