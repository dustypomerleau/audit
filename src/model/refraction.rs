use crate::{
    bounded::Bounded,
    error::AppError,
    model::{Axis, Cyl, Sca},
};
use serde::{Deserialize, Serialize};
use std::range::RangeBounds;

bounded!(
    (RefCylPower, i32, -1000..=1000, 25),
    (RefSph, i32, -2000..=2000, 25),
);

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RefCyl {
    pub power: RefCylPower,
    pub axis: Axis,
}

impl Cyl<i32> for RefCyl {
    fn power(&self) -> i32 {
        self.power.inner()
    }

    fn axis(&self) -> Axis {
        self.axis
    }
}

impl RefCyl {
    pub fn new(power: i32, axis: Axis) -> Result<Self, AppError> {
        Ok(Self {
            power: RefCylPower::new(power)?,
            axis,
        })
    }
}

/// A patient's subjective refraction.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Refraction {
    pub sph: RefSph,
    pub cyl: Option<RefCyl>,
}

impl Sca<i32> for Refraction {
    fn sph(&self) -> i32 {
        self.sph.inner()
    }

    fn cyl(&self) -> Option<impl Cyl<i32>> {
        self.cyl
    }
}

/// The preoperative and postoperative refractions for a given [`Case`](crate::case::Case).
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OpRefraction {
    pub before: Refraction,
    pub after: Refraction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_ref_sph() {
        assert!(RefSph::new(-500).is_ok());
    }

    #[test]
    fn out_of_bounds_ref_sph_returns_err() {
        assert!(RefSph::new(-2025).is_err());
        assert!(RefSph::new(2025).is_err());
    }

    #[test]
    fn nonzero_rem_ref_sph_returns_err() {
        assert!(RefSph::new(-510).is_err());
    }

    #[test]
    fn makes_ref_cyl_power() {
        assert!(RefCylPower::new(500).is_ok());
    }

    #[test]
    fn out_of_bounds_ref_cyl_power_returns_err() {
        assert!(RefCylPower::new(-1025).is_err());
        assert!(RefCylPower::new(1025).is_err());
    }

    #[test]
    fn nonzero_rem_iol_ref_cyl_power_returns_err() {
        assert!(RefCylPower::new(120).is_err());
    }
}
