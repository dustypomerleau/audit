use crate::{cyl::Cyl, distance::Distance, sca::Sca};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RefBoundsError {
    #[error(
        "refraction spherical power must be a value between -20 D and +20 D (supplied value: {0})"
    )]
    Sph(f32),

    #[error(
        "refraction cylinder power must be a value between -10 D and +10 D (supplied value: {0})"
    )]
    Cyl(f32),
}

#[derive(Debug, PartialEq)]
pub struct Refraction(Sca);

impl TryFrom<Sca> for Refraction {
    type Error = RefBoundsError;

    fn try_from(sca: Sca) -> Result<Self, Self::Error> {
        let Sca { sph, cyl } = sca;

        if (-20.0..=20.0).contains(&sph) && sph % 0.25 == 0.0 {
            match cyl {
                Some(Cyl { power, axis: _ }) => {
                    if (-10.0..=10.0).contains(&power) && power % 0.25 == 0.0 {
                        Ok(Self(sca))
                    } else {
                        Err(RefBoundsError::Cyl(power))
                    }
                }

                None => Ok(Self(sca)),
            }
        } else {
            Err(RefBoundsError::Sph(sph))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OpRefraction {
    pub before: Distance<Refraction>,
    pub after: Distance<Refraction>,
}
