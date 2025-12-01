use audit_macro::RangeBounded;
use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
use crate::model::Axis;

/// The class of [`Iol`] (monofocal, EDOF, multifocal).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum Focus {
    #[default]
    Mono,
    Edof,
    Multi,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct IolSe(
    #[bounded(range = -2000..=6000, rem = 25, default = 2000, mock_range = -200..=3000)] i32,
);

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct ToricPower(
    #[bounded(range = 100..=2000, rem = 25, default = 100, mock_range = 100..=600)] u32,
);

/// A specific model of IOL.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Iol {
    pub model: String,
    pub name: Option<String>,
    pub company: Option<String>,
    pub focus: Focus,
    pub toric: Option<ToricPower>,
}

/// The IOL for a particular [`Case`](crate::case::Case). Includes both the model and the specific
/// power chosen for this patient.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OpIol {
    pub iol: Iol,
    pub se: IolSe,
    // NOTE: It's theoretically possible to add an Axis to a case with a nontoric Iol, but the
    // cases selected for analysis of the Axis will be filtered by `self.iol.toric.is_some()`.
    // Using an enum that separates OpIols containing NontoricIol from those containing ToricIol is
    // possible, but in practice is not as ergonomic as it sounds.
    pub axis: Option<Axis>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_iol_se() {
        assert!(IolSe::new(2225).is_ok());
    }

    #[test]
    fn makes_toric_power() {
        assert!(ToricPower::new(225).is_ok());
    }

    #[test]
    fn out_of_bounds_iol_se_returns_err() {
        assert!(IolSe::new(-2025).is_err());
        assert!(IolSe::new(6025).is_err());
    }

    #[test]
    fn nonzero_rem_iol_se_returns_err() {
        assert!(IolSe::new(2001).is_err());
    }

    #[test]
    fn out_of_bounds_iol_toric_power_returns_err() {
        assert!(ToricPower::new(2025).is_err());
        assert!(ToricPower::new(75).is_err());
    }

    #[test]
    fn nonzero_rem_iol_toric_power_returns_err() {
        assert!(ToricPower::new(520).is_err());
    }
}
