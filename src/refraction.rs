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
        "refraction spherical power must be a value between -2000 and +2000 (supplied value: {0})"
    )]
    Sph(i32),

    #[error(
        "refraction cylinder power must be a value between -1000 and +1000 (supplied value: {0})"
    )]
    Cyl(i32),
}

/// A patient's subjective refraction. At initialization, the values are not yet bounds-checked. We
/// allow [`ScaMut`] methods only on the [`Unchecked`] variant (meaning, before bounds-checking).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Refraction<Bounds = Unchecked> {
    pub sph: i32,
    pub cyl: Option<Cyl>,
    pub bounds: PhantomData<Bounds>,
}

impl<Bounds> Sca for Refraction<Bounds> {
    fn sph(&self) -> i32 {
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

        if (-2000..=2000).contains(&sph) && sph % 25 == 0 {
            if let Some(Cyl { power, .. }) = cyl {
                if (-1000..=1000).contains(&power) && power % 25 == 0 {
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
    fn set_sph(mut self, sph: i32) -> Self {
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
            sph: -325,
            cyl: Some(Cyl {
                power: -75,
                axis: 100,
            }),
            bounds: PhantomData,
        };

        let output = unchecked.check().unwrap();

        let expected = Refraction::<Checked> {
            sph: -325,
            cyl: Some(Cyl {
                power: -75,
                axis: 100,
            }),
            bounds: PhantomData,
        };

        assert_eq!(output, expected);
    }

    #[test]
    fn out_of_bounds_refraction_sph_fails_check() {
        let sph = -4050;
        let refraction = RawSca::new(sph, Cyl::new(-25, 30).ok())
            .into_refraction_unchecked()
            .check();

        assert_eq!(refraction, Err(RefractionBoundsError::Sph(sph)));
    }

    #[test]
    fn nonzero_rem_refraction_sph_fails_check() {
        let sph = -1020;
        let refraction = RawSca::new(sph, Cyl::new(-25, 30).ok())
            .into_refraction_unchecked()
            .check();

        assert_eq!(refraction, Err(RefractionBoundsError::Sph(sph)));
    }

    #[test]
    fn out_of_bounds_refraction_cyl_power_fails_check() {
        let power = -1225;
        let refraction = RawSca::new(350, Cyl::new(power, 160).ok())
            .into_refraction_unchecked()
            .check();

        assert_eq!(refraction, Err(RefractionBoundsError::Cyl(power)));
    }

    #[test]
    fn nonzero_rem_refraction_cyl_power_fails_check() {
        let power = -60;
        let refraction = RawSca::new(350, Cyl::new(power, 160).ok())
            .into_refraction_unchecked()
            .check();

        assert_eq!(refraction, Err(RefractionBoundsError::Cyl(power)));
    }
}
