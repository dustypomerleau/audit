use crate::{
    cyl::{Cyl, CylPair},
    power::Power,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IolBoundsError {
    #[error("IOL must always have a spherical equivalent, but `None` was supplied")]
    NoSe,

    #[error("IOL cylinder must have both a power and an axis but the {0:?} was not supplied")]
    NoPair(CylPair),

    #[error("IOL spherical equivalent must be a multiple of 0.25 D between -20 D and +60 D (supplied value: {0})")]
    Se(f32),

    #[error(
        "IOL cylinder must be a multiple of 0.25 D between +1 D and +20 D (supplied value: {0})"
    )]
    Cyl(f32),

    #[error("IOL axis must be an integer value between 0° and 179° (supplied value: {0})")]
    Axis(i32),
}

#[derive(Debug, PartialEq)]
pub struct Iol {
    pub se: f32,
    pub cyl: Option<Cyl>,
}

impl TryFrom<Power> for Iol {
    type Error = IolBoundsError;

    fn try_from(power: Power) -> Result<Self, Self::Error> {
        let Power { sph: se, cyl } = power;

        if (-20.0..=60.0).contains(&se) && se % 0.25 == 0.0 {
            match cyl {
                Some(Cyl { power, axis: _ }) => {
                    if (1.0..=20.0).contains(&power) && power % 0.25 == 0.0 {
                        Ok(Self { se, cyl })
                    } else {
                        Err(IolBoundsError::Cyl(power))
                    }
                }

                None => Ok(Self { se, cyl: None }),
            }
        } else {
            Err(IolBoundsError::Se(se))
        }
    }
}
