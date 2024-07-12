use crate::{
    check::{BoundsCheck, Checked, Unchecked},
    cyl::{Cyl, CylPair},
    sca::{Sca, ScaMut},
};
// use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use thiserror::Error;

/// The error type for an invalid [`Iol`].
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

    #[error("incomplete IOL: IOL description must contain a model, name, focus (monofocal, EDOF, multifocal), and toric (true/false)")]
    Iol,
}

/// The class of [`Iol`] (monofocal, EDOF, multifocal)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Focus {
    Mono,
    Edof,
    Multi,
}

/// A specific model of IOL
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Iol {
    // todo: I would eventually prefer for this to be an enum, with IOL models explicitly allowed.
    pub company: String,
    pub model: String,
    pub name: String,
    pub focus: Focus,
    pub toric: bool,
}

/// The IOL for a particular [`Case`](crate::case::Case). Includes both the model and the specific
/// power chosen for this patient.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OpIol<Bounds = Unchecked> {
    /// An optional string, provided by the surgeon, to name/describe the IOL.
    pub iol: Iol,
    pub se: f32,
    pub cyl: Option<Cyl>,
    pub bounds: PhantomData<Bounds>,
}

impl BoundsCheck for OpIol<Unchecked> {
    type CheckedOutput = OpIol<Checked>;
    type Error = IolBoundsError;

    fn check(self) -> Result<Self::CheckedOutput, Self::Error> {
        let OpIol { se, cyl, .. } = self;

        let checked = OpIol::<Checked> {
            bounds: PhantomData,
            ..self
        };

        if (-20.0..=60.0).contains(&se) && se % 0.25 == 0.0 {
            if let Some(Cyl { power, .. }) = cyl {
                if (1.0..=20.0).contains(&power) && power % 0.25 == 0.0 {
                    Ok(checked)
                } else {
                    Err(IolBoundsError::Cyl(power))
                }
            } else {
                Ok(checked)
            }
        } else {
            Err(IolBoundsError::Se(se))
        }
    }
}

impl<Bounds> Sca for OpIol<Bounds> {
    fn sph(&self) -> f32 {
        self.se
    }

    fn cyl(&self) -> Option<Cyl> {
        self.cyl
    }
}

impl ScaMut for OpIol<Unchecked> {
    fn set_sph(mut self, sph: f32) -> Self {
        self.se = sph;
        self
    }

    fn set_cyl(mut self, cyl: Option<Cyl>) -> Self {
        self.cyl = cyl;
        self
    }
}

impl OpIol<Unchecked> {
    pub fn new<T: Sca>(iol: Iol, sca: T) -> Self {
        Self {
            iol,
            se: sca.sph(),
            cyl: sca.cyl(),
            bounds: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{axis::Axis, sca::RawSca};

    // todo: replace this function with an implementation of Mock(all)
    fn iol() -> Option<Iol> {
        Some(Iol {
            company: "Johnson & Johnson (TECNIS)".to_string(),
            model: "ZXTxxx".to_string(),
            name: "Tecnis Symfony".to_string(),
            focus: Focus::Edof,
            toric: true,
        })
    }

    #[test]
    fn makes_new_opiol() {
        let sca = RawSca::new(24.25, Some(3.0), Some(12)).unwrap();
        let checked = OpIol::new(iol(), sca).check().unwrap();

        assert_eq!(
            checked,
            OpIol::<Checked> {
                iol: iol(),
                se: 24.25,
                cyl: Some(Cyl {
                    power: 3.0,
                    axis: Axis(12)
                }),
                bounds: PhantomData
            }
        );
    }

    #[test]
    fn out_of_bounds_iol_se_returns_err() {
        // todo: randomize the out of bounds values on all failing tests
        // (Axis, Cyl, Iol, Refraction, Sca, Sia, Target, Va)
        let sca = RawSca::new(100.25, Some(3.0), Some(12)).unwrap();
        let checked = OpIol::new(iol(), sca).check();

        assert_eq!(checked, Err(IolBoundsError::Se(sca.sph())))
    }

    #[test]
    fn nonzero_rem_iol_se_returns_err() {
        let sca = RawSca::new(10.35, Some(3.0), Some(12)).unwrap();
        let opiol = OpIol::new(iol(), sca).check();

        assert_eq!(opiol, Err(IolBoundsError::Se(sca.sph())))
    }

    #[test]
    fn out_of_bounds_iol_cyl_power_returns_err() {
        let sca = RawSca::new(18.5, Some(31.0), Some(170)).unwrap();
        let cyl = sca.cyl().unwrap().power;
        let opiol = OpIol::new(iol(), sca).check();

        assert_eq!(opiol, Err(IolBoundsError::Cyl(cyl)))
    }

    #[test]
    fn nonzero_rem_iol_cyl_power_returns_err() {
        let sca = RawSca::new(28.5, Some(2.06), Some(170)).unwrap();
        let cyl = sca.cyl().unwrap().power;
        let opiol = OpIol::new(iol(), sca).check();

        assert_eq!(opiol, Err(IolBoundsError::Cyl(cyl)))
    }
}
