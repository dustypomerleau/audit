use crate::{
    cyl::{Cyl, CylPair},
    sca::Sca,
};
use thiserror::Error;

// We don't provide IolBoundsError::Axis(i32), because this error would already be thrown during
// construction of the wrapped Sca.
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
}

#[derive(Debug, PartialEq)]
pub struct Iol(Sca);

impl TryFrom<Sca> for Iol {
    type Error = IolBoundsError;

    fn try_from(sca: Sca) -> Result<Self, Self::Error> {
        let Sca { sph, cyl } = sca;

        if (-20.0..=60.0).contains(&sph) && sph % 0.25 == 0.0 {
            if cyl.is_some() && (!(1.0..=20.0).contains(&cyl.power) || !cyl.power % 0.25 == 0.0) {
                return Err(IolBoundsError::Cyl(power));
            }

            Ok(Self(sca))
        } else {
            Err(IolBoundsError::Se(sph))
        }
    }
}
