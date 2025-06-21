use crate::{
    bounded::Bounded,
    model::{Axis, Cyl, Sca},
};
use serde::{Deserialize, Serialize};
use std::range::RangeBounds;

/// A formula for calculating IOL power from biometry.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Formula {
    AscrsKrs,
    Barrett,
    BarrettTrueK,
    Evo,
    Haigis,
    HaigisL,
    HillRbf,
    HofferQ,
    Holladay1,
    Holladay2,
    Kane,
    Okulix,
    Olsen,
    SrkT,
    Other,
}

impl Default for Formula {
    fn default() -> Self {
        Self::Other
    }
}

impl Formula {
    pub fn is_thick(&self) -> bool {
        matches!(
            self,
            Self::AscrsKrs
                | Self::Barrett
                | Self::BarrettTrueK
                | Self::Evo
                | Self::HillRbf
                | Self::Holladay2
                | Self::Kane
                | Self::Okulix
                | Self::Olsen
        )
    }
}

// note: ToricPower, TargetCylPower are nonnegative, but RefCylPower can be negative.
// This has implications for the `Cyl` trait that you need to consider.
bounded!((TargetCylPower, u32, 0..=600), (TargetSe, i32, -600..=200));

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TargetCyl {
    pub power: TargetCylPower,
    pub axis: Axis,
}

impl Cyl<u32> for TargetCyl {
    fn power(&self) -> u32 {
        self.power.inner()
    }

    fn axis(&self) -> Axis {
        self.axis
    }
}

impl TargetCyl {
    pub fn new(power: TargetCylPower, axis: Axis) -> Self {
        Self { power, axis }
    }
}

/// The residual postop refraction for a case, assuming the provided formula and IOL constant.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Target {
    pub formula: Option<Formula>,
    pub custom_constant: bool,
    pub se: TargetSe,
    pub cyl: Option<TargetCyl>,
}

impl Sca<u32> for Target {
    fn sph(&self) -> i32 {
        self.se.inner()
    }

    fn cyl(&self) -> Option<impl Cyl<u32>> {
        self.cyl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_target_cyl_power() {
        assert!(TargetCylPower::new(10).is_ok());
    }

    #[test]
    fn out_of_bounds_target_cyl_power_returns_err() {
        assert!(TargetCylPower::new(601).is_err());
    }

    #[test]
    fn makes_target_se() {
        assert!(TargetSe::new(-10).is_ok());
    }

    #[test]
    fn out_of_bounds_target_se_returns_err() {
        assert!(TargetSe::new(-601).is_err());
        assert!(TargetSe::new(201).is_err());
    }
}
