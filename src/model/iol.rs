use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
use crate::model::Axis;
use crate::range_bounded;

/// The class of [`Iol`] (monofocal, EDOF, multifocal).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum Focus {
    #[default]
    Mono,
    Edof,
    Multi,
}

range_bounded!(
    (IolSe, i32, -2000..=6000, 25),
    (ToricPower, u32, 100..=2000, 25),
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
    // note: It's theoretically possible to add an Axis to a case with a nontoric Iol, but in
    // practice the cases selected for analysis of the Axis will be filtered by
    // `self.iol.toric.is_some()`.
    //
    // This can be fixed with private fields and an OpIol::new() function - see method in impl of
    // Mock.
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
