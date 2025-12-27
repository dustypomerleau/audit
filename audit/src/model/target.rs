use std::fmt::Display;

use audit_macro::RangeBounded;
use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
use crate::model::Axis;
use crate::model::Cyl;
use crate::model::Sca;

/// A formula for calculating IOL power from biometry.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "formula"))]
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
    #[default]
    Other,
}

impl Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AscrsKrs => write!(f, "AscrsKrs"),
            Self::Barrett => write!(f, "Barrett"),
            Self::BarrettTrueK => write!(f, "BarrettTrueK"),
            Self::Evo => write!(f, "Evo"),
            Self::Haigis => write!(f, "Haigis"),
            Self::HaigisL => write!(f, "HaigisL"),
            Self::HillRbf => write!(f, "HillRbf"),
            Self::HofferQ => write!(f, "HofferQ"),
            Self::Holladay1 => write!(f, "Holladay1"),
            Self::Holladay2 => write!(f, "Holladay2"),
            Self::Kane => write!(f, "Kane"),
            Self::Okulix => write!(f, "Okulix"),
            Self::Olsen => write!(f, "Olsen"),
            Self::SrkT => write!(f, "SrkT"),
            Self::Other => write!(f, "Other"),
        }
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

// NOTE: ToricPower, TargetCylPower are nonnegative, but RefCylPower can be negative.
// This has implications for the `Cyl` trait that you need to consider.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize,
)]
#[bounded(range = 0..=600, mock_range = 0..=75)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct TargetCylPower(i32);

#[derive(
    Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize,
)]
#[bounded(range = -600..=200, mock_range = -200..=20)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct TargetSe(i32);

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TargetCyl {
    pub power: TargetCylPower,
    pub axis: Axis,
}

impl Cyl for TargetCyl {
    type Power = TargetCylPower;

    fn power(&self) -> Self::Power { self.power }

    fn axis(&self) -> Axis { self.axis }
}

impl TargetCyl {
    pub fn new(power: TargetCylPower, axis: Axis) -> Self { Self { power, axis } }
}

/// The residual postop refraction for a case, assuming the provided formula and IOL constant.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Target {
    pub formula: Option<Formula>,
    pub custom_constant: bool,
    pub se: TargetSe,
    pub cyl: Option<TargetCyl>,
}

impl Sca for Target {
    type Sph = TargetSe;

    fn sph(&self) -> Self::Sph { self.se }

    fn cyl(&self) -> Option<impl Cyl> { self.cyl }
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
