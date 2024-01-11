use crate::{axis::Axis, cyl::Cyl, power::Power, va::Distance};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RefBoundsError {
    #[error("refraction must always have a spherical power component, but `None` was supplied")]
    NoSph,

    #[error(
        "refraction cylinder must have both a power and an axis but the {0:?} was not supplied"
    )]
    NoPair(Cyl),

    #[error(
        "refraction spherical power must be a value between -20 D and +20 D (supplied value: {0})"
    )]
    Sph(f32),

    #[error(
        "refraction cylinder power must be a value between -10 D and +10 D (supplied value: {0})"
    )]
    Cyl(f32),

    #[error("refraction axis must be an integer value between 0° and 179° (supplied value: {0})")]
    Axis(i32),
}

#[derive(Debug, PartialEq)]
pub struct Refraction {
    sph: f32,
    cyl: Option<Cyl>,
}

// Do the bounds check on Axis when you construct Power, because the axis check is identical for
// every type that you will try_into() from Power.
impl TryFrom<Power> for Refraction {
    type Error = RefBoundsError;

    fn try_from(power: Power) -> Result<Self, Self::Error> {
        let Power { sph, cyl } = power;

        if (-20.0..=20.0).contains(&sph) && sph % 0.25 == 0.0 {
            match cyl {
                Some(Cyl { power, axis }) => {
                    if (-10.0..=10.0).contains(&power) && power % 0.25 == 0.0 {
                        Ok(Self {
                            sph,
                            cyl: Some(Cyl { power, axis }),
                        })
                    }
                }
                None => Ok(Self { sph, cyl: None }),
            }
        } else {
            RefBoundsError::Sph(sph)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OpRefraction {
    before: Distance<Refraction>,
    after: Distance<Refraction>,
}
