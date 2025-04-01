use crate::{
    bounded::Bounded,
    cyl::{Axis, Cyl},
};
use serde::{Deserialize, Serialize};
use std::range::RangeBounds;

bounded!((SiaPower, u32, 0..=200));

/// A surgically-induced astigmatism. The purist would prefer using
/// `meridian` rather than `axis` for [`Sia`] and biometric Ks, but on balance I've
/// decided that the cognitive overhead of using both terms in the code is higher than the cognitive
/// overhead of knowing when `axis` actually refers to a meridian.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Sia {
    pub power: SiaPower,
    pub axis: Axis,
}

impl Cyl<u32> for Sia {
    fn power(&self) -> u32 {
        self.power.inner()
    }

    fn axis(&self) -> Axis {
        self.axis
    }
}

impl Sia {
    /// Create a new bounds-checked [`Sia`].
    pub fn new(power: SiaPower, axis: Axis) -> Self {
        Self { power, axis }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds_sia_power_returns_err() {
        assert!(SiaPower::new(201).is_err());
    }
}
