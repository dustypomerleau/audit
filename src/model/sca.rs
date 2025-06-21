use crate::{
    bounded::{Bounded, BoundsError},
    model::{
        Cyl, CylPower, Formula, RawCyl, RefCyl, RefSph, Refraction, Target, TargetCyl,
        TargetCylPower, TargetSe,
    },
};
use serde::{Deserialize, Serialize};

/// A type that wraps a sphere and a cylinder.
pub trait Sca<T>
where T: CylPower
{
    /// Return the spherical value from a [`Sca`].
    fn sph(&self) -> i32;

    /// Return the [`Cyl`] from a [`Sca`].
    fn cyl(&self) -> Option<impl Cyl<T>>;
}

pub fn into_target<T, U>(
    sca: U,
    formula: Option<Formula>,
    custom_constant: bool,
) -> Result<Target, BoundsError>
where
    T: CylPower + Into<u32>,
    U: Sca<T>,
{
    let cyl = if let Some(cyl) = sca.cyl() {
        Some(TargetCyl {
            power: TargetCylPower::new(cyl.power().into())?,
            axis: cyl.axis(),
        })
    } else {
        None
    };

    Ok(Target {
        formula,
        custom_constant,
        se: TargetSe::new(sca.sph())?,
        cyl,
    })
}

pub fn into_refraction<T, U>(sca: U) -> Result<Refraction, BoundsError>
where
    T: CylPower + Into<i32>,
    U: Sca<T>,
{
    let cyl = if let Some(cyl) = sca.cyl() {
        Some(RefCyl::new(cyl.power().into(), cyl.axis())?)
    } else {
        None
    };

    Ok(Refraction {
        sph: RefSph::new(sca.sph())?,
        cyl,
    })
}

/// A primitive type wrapping a sphere and a cylinder. Can be passed to [`Sca`] constructors that
/// apply bounds checking and return a more specific type.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct RawSca {
    pub sph: i32,
    pub cyl: Option<RawCyl>,
}

impl Sca<i32> for RawSca {
    fn sph(&self) -> i32 {
        self.sph
    }

    fn cyl(&self) -> Option<impl Cyl<i32>> {
        self.cyl
    }
}

impl RawSca {
    /// Construct a new [`RawSca`].
    pub fn new(sph: i32, cyl: Option<RawCyl>) -> Self {
        Self { sph, cyl }
    }
}

mod tests {}
