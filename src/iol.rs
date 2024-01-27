use crate::{
    cyl::{Cyl, CylPair},
    sca::Sca,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// We don't provide IolBoundsError::Axis(i32), because this error would already be thrown during
// construction of the wrapped Sca.
#[derive(Debug, Error, PartialEq)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// todo: see your DB schema - you need Iol as a full struct with several components, and
// Iol::new() instead of TryFrom<Sca>, and then OpIol wrapping for Case
pub struct Iol(pub Sca);

impl TryFrom<Sca> for Iol {
    type Error = IolBoundsError;

    fn try_from(sca: Sca) -> Result<Self, Self::Error> {
        let Sca { sph, cyl } = sca;

        if (-20.0..=60.0).contains(&sph) && sph % 0.25 == 0.0 {
            match cyl {
                Some(Cyl { power, axis: _ }) => {
                    if (1.0..=20.0).contains(&power) && power % 0.25 == 0.0 {
                        Ok(Self(sca))
                    } else {
                        Err(IolBoundsError::Cyl(power))
                    }
                }

                None => Ok(Self(sca)),
            }
        } else {
            Err(IolBoundsError::Se(sph))
        }
    }
}

mod tests {
    use super::*;
    use crate::axis::Axis;

    #[test]
    fn iol_implements_try_from_sca() {
        let iol: Iol = Sca::new(24.25, Some(3.0), Some(12))
            .unwrap()
            .try_into()
            .unwrap();

        assert_eq!(
            iol,
            Iol(Sca {
                sph: 24.25,
                cyl: Some(Cyl {
                    power: 3.0,
                    axis: Axis(12)
                })
            })
        )
    }

    #[test]
    fn out_of_bounds_iol_se_returns_err() {
        let se = 100.25f32;
        let iol: Result<Iol, IolBoundsError> =
            Sca::new(se, Some(3.0), Some(12)).unwrap().try_into();

        assert_eq!(iol, Err(IolBoundsError::Se(se)))
    }

    #[test]
    fn nonzero_rem_iol_se_returns_err() {
        let se = 10.35f32;
        let iol: Result<Iol, IolBoundsError> =
            Sca::new(se, Some(3.0), Some(12)).unwrap().try_into();

        assert_eq!(iol, Err(IolBoundsError::Se(se)))
    }

    #[test]
    fn out_of_bounds_iol_cyl_power_returns_err() {
        let cyl = 31.0f32;
        let iol: Result<Iol, IolBoundsError> =
            Sca::new(12.5, Some(cyl), Some(12)).unwrap().try_into();

        assert_eq!(iol, Err(IolBoundsError::Cyl(cyl)))
    }

    #[test]
    fn nonzero_rem_iol_cyl_power_returns_err() {
        let cyl = 2.06f32;
        let iol: Result<Iol, IolBoundsError> =
            Sca::new(12.5, Some(cyl), Some(12)).unwrap().try_into();

        assert_eq!(iol, Err(IolBoundsError::Cyl(cyl)))
    }
}
