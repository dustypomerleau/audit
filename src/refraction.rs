use crate::{
    bounds_check::{BoundsCheck, Checked, Unchecked},
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

/// A patient's subjective refraction. At initialization, the values are not yet bounds-checked. We
/// allow [`ScaMut`] methods only on the [`Unchecked`] variant (meaning, before bounds-checking).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Refraction<Bounds = Unchecked> {
    pub sph: f32,
    pub cyl: Option<Cyl>,
    pub bounds: PhantomData<Bounds>,
}

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

        let checked = Refraction::<Checked> {
            bounds: PhantomData,
            ..self
        };

        if (-20.0..=20.0).contains(&sph) && sph % 0.25 == 0.0 {
            if let Some(Cyl { power, .. }) = cyl {
                if (-10.0..=10.0).contains(&power) && power % 0.25 == 0.0 {
                    Ok(checked)
                } else {
                    Err(RefractionBoundsError::Cyl(power))
                }
            } else {
                Ok(checked)
            }
        } else {
            Err(RefractionBoundsError::Sph(sph))
        }
    }
}

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
    /// Create a new [`Refraction`] from a generic [`Sca`]. At initialization, the values are not
    /// yet bounds-checked. We allow [`ScaMut`] methods only on the [`Unchecked`] variant
    /// (meaning, before bounds-checking).
    pub fn new<T: Sca>(sca: T) -> Self {
        Refraction {
            sph: sca.sph(),
            cyl: sca.cyl(),
            bounds: PhantomData,
        }
    }
}

impl TryFrom<Refraction<Unchecked>> for Refraction<Checked> {
    type Error = RefractionBoundsError;

    fn try_from(refraction: Refraction<Unchecked>) -> Result<Self, Self::Error> {
        refraction.check()
    }
}

/// The preoperative and postoperative refractions for a given [`Case`](crate::case::Case).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OpRefraction {
    pub before: Refraction<Checked>,
    pub after: Refraction<Checked>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sca::RawSca;

    #[test]
    fn check_succeeds_on_in_bounds_refraction() {
        let unchecked = Refraction::<Unchecked> {
            sph: -3.25,
            cyl: Some(Cyl {
                power: -0.75,
                axis: 100,
            }),
            bounds: PhantomData,
        };

        let output = unchecked.check().unwrap();

        let expected = Refraction::<Checked> {
            sph: -3.25,
            cyl: Some(Cyl {
                power: -0.75,
                axis: 100,
            }),
            bounds: PhantomData,
        };

        assert_eq!(output, expected);
    }

    #[test]
    fn out_of_bounds_refraction_sph_fails_check() {
        let sph = -40.5f32;
        let refraction = RawSca::new(sph, Some(-0.25), Some(30))
            .unwrap()
            .into_refraction()
            .check();

        assert_eq!(refraction, Err(RefractionBoundsError::Sph(sph)));
    }

    #[test]
    fn nonzero_rem_refraction_sph_fails_check() {
        let sph = -10.2f32;
        let refraction = RawSca::new(sph, Some(-0.25), Some(30))
            .unwrap()
            .into_refraction()
            .check();

        assert_eq!(refraction, Err(RefractionBoundsError::Sph(sph)));
    }

    #[test]
    fn out_of_bounds_refraction_cyl_power_fails_check() {
        let cyl = -12.25f32;
        let refraction = RawSca::new(3.5, Some(cyl), Some(160))
            .unwrap()
            .into_refraction()
            .check();

        assert_eq!(refraction, Err(RefractionBoundsError::Cyl(cyl)));
    }

    #[test]
    fn nonzero_rem_refraction_cyl_power_fails_check() {
        let cyl = -0.6f32;
        let refraction = RawSca::new(3.5, Some(cyl), Some(160))
            .unwrap()
            .into_refraction()
            .check();

        assert_eq!(refraction, Err(RefractionBoundsError::Cyl(cyl)));
    }
}
