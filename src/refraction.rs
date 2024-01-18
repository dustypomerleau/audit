use crate::{cyl::Cyl, distance::Distance, sca::Sca};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
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

mod tests {
    use super::*;
    use crate::axis::Axis;

    #[test]
    fn refraction_implements_try_from_sca() {
        let refraction: Refraction = Sca::new(-3.25, Some(-0.75), Some(100))
            .unwrap()
            .try_into()
            .unwrap();

        assert_eq!(
            refraction,
            Refraction(Sca {
                sph: -3.25,
                cyl: Some(Cyl {
                    power: -0.75,
                    axis: Axis(100)
                })
            })
        )
    }

    #[test]
    fn out_of_bounds_refraction_sph_returns_err() {
        let refraction: Result<Refraction, RefBoundsError> =
            Sca::new(-40.5, Some(-0.25), Some(30)).unwrap().try_into();

        assert_eq!(refraction, Err(RefBoundsError::Sph(-40.5)))
    }

    #[test]
    fn nonzero_rem_refraction_sph_returns_err() {
        let refraction: Result<Refraction, RefBoundsError> =
            Sca::new(-10.2, Some(-0.25), Some(30)).unwrap().try_into();

        assert_eq!(refraction, Err(RefBoundsError::Sph(-10.2)))
    }

    #[test]
    fn out_of_bounds_refraction_cyl_power_returns_err() {
        let refraction: Result<Refraction, RefBoundsError> =
            Sca::new(3.5, Some(-12.25), Some(160)).unwrap().try_into();

        assert_eq!(refraction, Err(RefBoundsError::Cyl(-12.25)))
    }

    #[test]
    fn nonzero_rem_refraction_cyl_power_returns_err() {
        let refraction: Result<Refraction, RefBoundsError> =
            Sca::new(3.5, Some(-0.6), Some(160)).unwrap().try_into();

        assert_eq!(refraction, Err(RefBoundsError::Cyl(-0.6)))
    }
}
