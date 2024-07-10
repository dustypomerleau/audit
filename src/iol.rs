use crate::{
    check::{BoundsCheck, Checked, Unchecked},
    cyl::{Cyl, CylPair},
    sca::{RawSca, Sca, ScaMut},
};
use edgedb_derive::Queryable;
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
#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub enum Focus {
    Mono,
    Edof,
    Multi,
}

/// A specific model of IOL
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Iol {
    // todo: I would eventually prefer for this to be an enum, with IOL models explicitly allowed.
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
    pub surgeon_label: Option<String>,
    pub iol: Option<Iol>,
    pub se: f32,
    pub cyl: Option<Cyl>,
    pub bounds: PhantomData<Bounds>,
}

impl BoundsCheck for OpIol<Unchecked> {
    type Error = IolBoundsError;
    type Output = OpIol<Checked>;

    fn check(self) -> Result<Self::Output, Self::Error> {
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
    pub fn new<T: Sca>(surgeon_label: Option<String>, iol: Option<Iol>, sca: T) -> Self {
        Self {
            surgeon_label,
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

    // todo: replace this function with an implementation of Mock(all)
    fn iol() -> Option<Iol> {
        Some(Iol {
            model: "ZXTxxx".to_string(),
            name: "Tecnis Symfony".to_string(),
            focus: Focus::Edof,
            toric: true,
        })
    }

    #[test]
    fn makes_new_opiol() {
        // let iol = iol();
        // let sca = Sca::new(24.25, Some(3.0), Some(12)).unwrap();
        // let opiol = OpIol::new(Some("sn60wf".to_string()), iol.clone(), sca).unwrap();
        //
        // assert_eq!(
        //     opiol,
        //     OpIol {
        //         surgeon_label: Some("sn60wf".to_string()),
        //         iol,
        //         sca
        //     }
        // )
    }

    #[test]
    fn out_of_bounds_iol_se_returns_err() {
        // todo: randomize the out of bounds values on all failing tests
        // (Axis, Cyl, Iol, Refraction, Sca, Sia, Target, Va)
        let se = 100.25;
        let iol = iol();
        let cyl = Cyl::new(3.0, 12).unwrap();
        let opiol = OpIol {
            surgeon_label: None,
            iol,
            se,
            cyl: Some(cyl),
            bounds: PhantomData,
        };

        assert_eq!(opiol.check(), Err(IolBoundsError::Se(se)))
    }

    #[test]
    fn nonzero_rem_iol_se_returns_err() {
        let se = 10.35;
        let iol = iol();
        let sca = Sca::new(se, Some(3.0), Some(12)).unwrap();
        let opiol = OpIol::new(Some("sn60wf".to_string()), iol, sca);

        assert_eq!(opiol, Err(IolBoundsError::Se(se)))
    }

    #[test]
    fn out_of_bounds_iol_cyl_power_returns_err() {
        let cyl = 31.0;
        let iol = iol();
        let sca = Sca::new(18.5, Some(cyl), Some(170)).unwrap();
        let opiol = OpIol::new(Some("sn60wf".to_string()), iol, sca);

        assert_eq!(opiol, Err(IolBoundsError::Cyl(cyl)))
    }

    #[test]
    fn nonzero_rem_iol_cyl_power_returns_err() {
        let cyl = 2.06;
        let iol = iol();
        let sca = Sca::new(28.5, Some(cyl), Some(170)).unwrap();
        let opiol = OpIol::new(Some("sn60wf".to_string()), iol, sca);

        assert_eq!(opiol, Err(IolBoundsError::Cyl(cyl)))
    }
}
