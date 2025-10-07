use crate::{
    bounded::Bounded,
    error::AppError,
    model::{Axis, Cyl, Sca},
    range_bounded,
};
use serde::{Deserialize, Serialize};

range_bounded!(
    (RefCylPower, i32, -1000..=1000, 25),
    (RefSph, i32, -2000..=2000, 25),
);

/// A marker trait for measurements with the following properties:
///
/// 1. They are taken in the spectacle plane
/// 2. They are stored in integer form, as (diopters * 100)
pub trait Spectacle: Bounded<Idx = i32> {}

impl Spectacle for RefCylPower {}
impl Spectacle for RefSph {}

/// Types that can be vertexed from their plane to the corneal plane (from either the spectacle or
/// IOL planes).
pub trait VertexK {
    /// Returns the power in diopters at the corneal plane. Be sure to convert to diopters for any
    /// value stored in the DB as (diopters * 100).
    ///
    /// In general:
    ///
    /// Power(corneal plane) =
    ///     Power(spectacle plane) / (1 - (Power(spectacle plane)(Vertex distance in meters)))
    ///
    /// For a default vertex distance of 13 mm we get:
    ///
    /// Kpower = Spower / (1 - (Spower * 0.013))
    fn vertex(&self) -> f64;
}

impl<T: Spectacle> VertexK for T {
    fn vertex(&self) -> f64 {
        let spectacle = self.inner() as f64 / 100.0;

        spectacle / (1.0 - (spectacle * 0.013))
    }
}

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
