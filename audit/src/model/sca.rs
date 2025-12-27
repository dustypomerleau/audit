use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
use crate::error::AppError;
use crate::model::Cyl;
use crate::model::Formula;
use crate::model::RawCyl;
use crate::model::RefCyl;
use crate::model::RefCylPower;
use crate::model::RefSph;
use crate::model::Refraction;
use crate::model::Target;
use crate::model::TargetCyl;
use crate::model::TargetCylPower;
use crate::model::TargetSe;

/// A type that wraps a sphere and a cylinder.
pub trait Sca {
    type Sph;

    /// Return the spherical value from a [`Sca`].
    fn sph(&self) -> Self::Sph;

    /// Return the [`Cyl`] from a [`Sca`].
    fn cyl(&self) -> Option<impl Cyl>;
}

/// A primitive type wrapping a sphere and a cylinder. Can be passed to [`Sca`] constructors that
/// apply bounds checking and return a more specific type.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct RawSca {
    pub sph: i32,
    pub cyl: Option<RawCyl>,
}

impl Sca for RawSca {
    type Sph = i32;

    fn sph(&self) -> Self::Sph { self.sph }

    fn cyl(&self) -> Option<impl Cyl> { self.cyl }
}

impl RawSca {
    /// Construct a new [`RawSca`].
    pub fn new(sph: i32, cyl: Option<RawCyl>) -> Self { Self { sph, cyl } }

    pub fn into_target(
        &self,
        formula: Option<Formula>,
        custom_constant: bool,
    ) -> Result<Target, AppError> {
        let cyl = if let Some(cyl) = self.cyl {
            Some(TargetCyl {
                power: TargetCylPower::new(cyl.power())?,
                axis: cyl.axis,
            })
        } else {
            None
        };

        Ok(Target {
            formula,
            custom_constant,
            se: TargetSe::new(self.sph)?,
            cyl,
        })
    }

    pub fn into_refraction(&self) -> Result<Refraction, AppError> {
        let cyl = if let Some(cyl) = self.cyl {
            Some(RefCyl::new(RefCylPower::new(cyl.power)?, cyl.axis))
        } else {
            None
        };

        Ok(Refraction {
            sph: RefSph::new(self.sph)?,
            cyl,
        })
    }
}

mod tests {}
