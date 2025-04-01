use crate::{
    bounded::Bounded,
    cyl::{Axis, Cyl},
};
use serde::{Deserialize, Serialize};
use std::ops::RangeBounds;

bounded!(
    (Acd, u32, 0..=600),
    (Al, u32, 1200..=3800),
    (Cct, u32, 350..=650),
    (Kpower, u32, 3000..=6500),
    (Lt, u32, 200..=800),
    (Wtw, u32, 800..=1400),
);

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct K {
    pub power: Kpower,
    pub axis: Axis,
}

impl Cyl<u32> for K {
    fn power(&self) -> u32 {
        self.power.inner()
    }

    fn axis(&self) -> Axis {
        self.axis
    }
}

impl K {
    pub fn new(power: Kpower, axis: Axis) -> Self {
        Self { power, axis }
    }
}

// These fields are private to enforce the invariant that flat <= steep.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Ks {
    flat: K,
    steep: K,
}

impl Ks {
    pub fn new(k1: K, k2: K) -> Self {
        if k1.power.inner() <= k2.power.inner() {
            Self {
                flat: k1,
                steep: k2,
            }
        } else {
            Self {
                flat: k2,
                steep: k1,
            }
        }
    }

    pub fn flat_power(&self) -> u32 {
        self.flat.power.inner()
    }

    pub fn steep_power(&self) -> u32 {
        self.steep.power.inner()
    }

    pub fn flat_axis(&self) -> u32 {
        self.flat.axis.inner()
    }

    pub fn steep_axis(&self) -> u32 {
        self.steep.axis.inner()
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Biometry {
    pub al: Al,
    pub ks: Ks,
    pub acd: Acd,
    pub lt: Lt,
    pub cct: Option<Cct>,
    pub wtw: Option<Wtw>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assigns_ks_correctly() {
        let ks = Ks::new(
            K::new(Kpower::new(4230).unwrap(), Axis::new(100).unwrap()),
            K::new(Kpower::new(4025).unwrap(), Axis::new(10).unwrap()),
        );

        assert!(ks.flat_power() < ks.steep_power());
    }
}
