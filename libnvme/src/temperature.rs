// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::num::NonZeroU16;

/// A temperature reported by an NVMe controller, which reports them in
/// Kelvin.
///
/// 0 K isn't a real reading. NVMe uses it to mean a temperature sensor isn't
/// implemented or a threshold isn't reported, so it can't be represented
/// here; [`Temperature::from_kelvin`] returns `None` for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(into = "TemperatureRepr", try_from = "TemperatureRepr")
)]
pub struct Temperature(NonZeroU16);

impl Temperature {
    /// `None` for 0 K.
    pub const fn from_kelvin(kelvin: u16) -> Option<Self> {
        match NonZeroU16::new(kelvin) {
            Some(k) => Some(Self(k)),
            None => None,
        }
    }

    pub fn kelvin(&self) -> u16 {
        self.0.get()
    }

    /// Whole degrees Celsius, using 273 as the offset as nvmeadm and nvme-cli
    /// do, since NVMe reports whole Kelvin.
    pub fn celsius(&self) -> i32 {
        i32::from(self.kelvin()) - 273
    }
}

/// Serialized form. `celsius` is for readers and is ignored when
/// deserializing.
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct TemperatureRepr {
    kelvin: u16,
    #[serde(skip_deserializing)]
    celsius: i32,
}

#[cfg(feature = "serde")]
impl From<Temperature> for TemperatureRepr {
    fn from(t: Temperature) -> Self {
        Self { kelvin: t.kelvin(), celsius: t.celsius() }
    }
}

#[cfg(feature = "serde")]
impl TryFrom<TemperatureRepr> for Temperature {
    type Error = &'static str;

    fn try_from(r: TemperatureRepr) -> Result<Self, Self::Error> {
        Temperature::from_kelvin(r.kelvin).ok_or("temperature of 0 K")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        assert_eq!(Temperature::from_kelvin(0), None);
        let t = Temperature::from_kelvin(308).unwrap();
        assert_eq!(t.kelvin(), 308);
        assert_eq!(t.celsius(), 35);
        assert_eq!(Temperature::from_kelvin(1).unwrap().celsius(), -272);
    }
}
