use crate::{
    check::{BoundsCheck, Checked, Unchecked},
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

// Reading values is allowed in both the `Unchecked` and `Checked` variants...
impl<Bounds> Sca for Refraction<Bounds> {
    fn sph(&self) -> f32 {
        self.sph
    }

    fn cyl(&self) -> Option<Cyl> {
        self.cyl
    }
}

impl BoundsCheck for Refraction<Unchecked> {
    type CheckedOutput = Refraction<Checked>;
    type Error = RefractionBoundsError;

    fn check(self) -> Result<Self::CheckedOutput, Self::Error> {
        let Self { sph, cyl, .. } = self;

        if (-20.0..=20.0).contains(&sph) && sph % 0.25 == 0.0 {
            let _ = if let Some(Cyl { power, .. }) = cyl {
                if (-10.0..=10.0).contains(&power) && power % 0.25 == 0.0 {
                } else {
                    return Err(RefractionBoundsError::Cyl(power));
                }
            };

            Ok(Refraction::<Checked> {
                bounds: PhantomData,
                ..self
            })
        } else {
            Err(RefractionBoundsError::Sph(sph))
        }
    }
}

// ...but writing to values is only allowed in `Unchecked`, essentially rendering `Checked`
// immutable once instantiated.
impl ScaMut for Refraction<Unchecked> {
    fn set_sph(mut self, sph: f32) -> Self {
        self.sph = sph;
        self
    }

    fn set_cyl(mut self, cyl: Option<Cyl>) -> Self {
        self.cyl = cyl;
        self
    }
}

impl Refraction<Unchecked> {
    pub fn new(sph: f32, cyl: Option<Cyl>) -> Self {
        Refraction {
            sph,
            cyl,
            bounds: PhantomData,
        }
    }
}

impl TryFrom<Refraction<Unchecked>> for Refraction<Checked> {
    type Error = RefractionBoundsError;

    fn try_from(value: Refraction<Unchecked>) -> Result<Self, Self::Error> {
        value.check()
    }
}

impl Refraction<Checked> {}

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
        let unchecked: Refraction<Unchecked> = Refraction {
            sph: -3.25,
            cyl: Some(Cyl {
                power: -0.75,
                axis: Axis(100),
            }),
            bounds: PhantomData,
        };

        let test = unchecked.try_into::<Refraction<Checked>>().unwrap();

        let output: Refraction<Checked> = unchecked.try_into().unwrap();

        let expected: Refraction<Checked> = Refraction {
            sph: -3.25,
            cyl: Some(Cyl {
                power: -0.75,
                axis: Axis(100),
            }),
            bounds: PhantomData,
        };

        assert_eq!(output, expected);
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
