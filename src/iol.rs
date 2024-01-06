use crate::{axis::Axis, cyl::Cyl};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IolBoundsError {
    #[error("IOL must always have a spherical equivalent, but `None` was supplied")]
    NoSe,

    #[error("IOL cylinder must have both a power and an axis but the {0:?} was not supplied")]
    NoPair(Cyl),

    #[error("IOL spherical equivalent must be a multiple of 0.25 between -20 and +60 (supplied value: {0})")]
    Se(f32),

    #[error("IOL cylinder must be a multiple of 0.25 between +1 and +20 (supplied value: {0})")]
    Cyl(f32),

    #[error("IOL axis must be an integer between 0 and 179 (supplied value: {0})")]
    Axis(i32),
}

#[derive(Debug, PartialEq)]
pub struct IolSePower(f32);

impl IolSePower {
    pub fn new(power: f32) -> Option<Self> {
        if (-20.0..=60.0).contains(&power) && power % 0.25 == 0.0 {
            Some(Self(power))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IolCylPower(f32);

impl IolCylPower {
    pub fn new(power: f32) -> Option<Self> {
        if (1.0..=20.0).contains(&power) && power % 0.25 == 0.0 {
            Some(Self(power))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IolCyl {
    power: IolCylPower,
    axis: Axis,
}

impl IolCyl {
    fn new(power: f32, axis: i32) -> Result<Self, IolBoundsError> {
        if let Some(power) = IolCylPower::new(power) {
            if let Some(axis) = Axis::new(axis) {
                Ok(Self { power, axis })
            } else {
                Err(IolBoundsError::Axis(axis))
            }
        } else {
            Err(IolBoundsError::Cyl(power))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Iol {
    se: IolSePower,
    cyl: Option<IolCyl>,
}

impl Iol {
    pub fn new(se: f32, cyl: Option<f32>, axis: Option<i32>) -> Result<Self, IolBoundsError> {
        if let Some(se) = IolSePower::new(se) {
            let cyl = match (cyl, axis) {
                (Some(cyl), Some(axis)) => Some(IolCyl::new(cyl, axis)?),

                (Some(_cyl), _) => return Err(IolBoundsError::NoPair(Cyl::Axis)),

                (_, Some(_axis)) => return Err(IolBoundsError::NoPair(Cyl::Power)),

                (_, _) => None,
            };

            Ok(Self { se, cyl })
        } else {
            Err(IolBoundsError::Se(se))
        }
    }
}
