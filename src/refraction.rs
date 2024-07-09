use crate::{
    axis::Axis,
    check::{Checked, Unchecked},
    cyl::Cyl,
    sca::{Sca, ScaMut},
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use thiserror::Error;

/// The error type for an invalid [`Refraction`].
#[derive(Debug, Error, PartialEq)]
pub enum RefractionBoundsError {
    #[error(
        "refraction spherical power must be a value between -20 D and +20 D (supplied value: {0})"
    )]
    Sph(f32),

    #[error(
        "refraction cylinder power must be a value between -10 D and +10 D (supplied value: {0})"
    )]
    Cyl(f32),
}

/// A patient's subjective refraction.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Refraction<Bounds = Unchecked> {
    sph: f32,
    cyl: Option<Cyl>,
    bounds: PhantomData<Bounds>,
}

impl<Bounds> Default for Refraction<Bounds> {
    fn default() -> Self {
        Self {
            sph: 0,
            cyl: Some(Cyl {
                power: 0,
                axis: Axis(0),
            }),
            bounds: PhantomData,
        }
    }
}

impl<Bounds> Sca for Refraction<Bounds> {
    fn sph(&self) -> f32 {
        self.sph
    }

    fn cyl(&self) -> Option<Cyl> {
        self.cyl
    }
}

impl ScaMut for Refraction<Unchecked> {
    fn set_sph(&mut self, sph: f32) -> Self {
        *self.sph = sph;
        self
    }

    fn set_cyl(&mut self, cyl: Cyl) -> Self {
        *self.cyl = Some(cyl);
        self
    }
}

impl Refraction<Checked> {}

impl<T: Sca> TryFrom<T> for Refraction<Checked> {
    type Error = RefractionBoundsError;

    fn try_from(t: T) -> Result<Self, Self::Error> {
        let (sph, cyl) = (t.sph(), t.cyl());

        // todo: from here on
        if (-20.0..=20.0).contains(&sph) && sph % 0.25 == 0.0 {
            let cyl = match cyl {
                Some(Cyl { power, .. }) => {
                    if (-10.0..=10.0).contains(&power) && power % 0.25 == 0.0 {
                        cyl
                    } else {
                        return Err(RefractionBoundsError::Cyl(power));
                    }
                }

                None => None,
            };

            Ok(Self(sca))
        } else {
            Err(RefractionBoundsError::Sph(sph))
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OpRefraction {
    pub before: Refraction<Checked>,
    pub after: Refraction<Checked>,
}

#[cfg(test)]
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
        let sph = -40.5f32;
        let refraction: Result<Refraction, RefBoundsError> =
            Sca::new(sph, Some(-0.25), Some(30)).unwrap().try_into();

        assert_eq!(refraction, Err(RefBoundsError::Sph(sph)))
    }

    #[test]
    fn nonzero_rem_refraction_sph_returns_err() {
        let sph = -10.2f32;
        let refraction: Result<Refraction, RefBoundsError> =
            Sca::new(sph, Some(-0.25), Some(30)).unwrap().try_into();

        assert_eq!(refraction, Err(RefBoundsError::Sph(sph)))
    }

    #[test]
    fn out_of_bounds_refraction_cyl_power_returns_err() {
        let cyl = -12.25f32;
        let refraction: Result<Refraction, RefBoundsError> =
            Sca::new(3.5, Some(cyl), Some(160)).unwrap().try_into();

        assert_eq!(refraction, Err(RefBoundsError::Cyl(cyl)))
    }

    #[test]
    fn nonzero_rem_refraction_cyl_power_returns_err() {
        let cyl = -0.6f32;
        let refraction: Result<Refraction, RefBoundsError> =
            Sca::new(3.5, Some(cyl), Some(160)).unwrap().try_into();

        assert_eq!(refraction, Err(RefBoundsError::Cyl(cyl)))
    }
}
