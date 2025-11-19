use audit_macro::RangeBounded;
use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
use crate::model::Axis;
use crate::model::Cyl;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct Acd(#[bounded(range = 0..=600)] u32);

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct Al(#[bounded(range = 1200..=3800)] u32);

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct Cct(#[bounded(range = 350..=650)] u32);

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct Kpower(#[bounded(range = 3000..=6500)] u32);

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct Lt(#[bounded(range = 200..=800)] u32);

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct Wtw(#[bounded(range = 800..=1400)] u32);

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct K {
    pub power: Kpower,
    pub axis: Axis,
}

impl Cyl<u32> for K {
    fn power(&self) -> u32 { self.power.inner() }

    fn axis(&self) -> Axis { self.axis }
}

impl K {
    pub fn new(power: Kpower, axis: Axis) -> Self { Self { power, axis } }
}

// Safety: These fields are private to enforce the invariant that flat <= steep.
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

    pub fn flat_power(&self) -> u32 { self.flat.power.inner() }

    pub fn steep_power(&self) -> u32 { self.steep.power.inner() }

    pub fn cyl(&self) -> u32 { self.steep_power() - self.flat_power() }

    pub fn flat_axis(&self) -> u32 { self.flat.axis.inner() }

    pub fn steep_axis(&self) -> u32 { self.steep.axis.inner() }
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
