use crate::{
    cyl::{Cyl, CylPair},
    sca::Sca,
    target::Formula,
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
pub enum Focus {
    Mono,
    Edof,
    Multi,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Iol {
    model: String,
    name: String,
    focus: Focus,
    toric: bool,
}

pub struct OpIol {
    iol: Iol,
    sca: Sca,
}

impl OpIol {
    fn new(iol: Iol, sca: Sca) -> Result<Self, IolBoundsError> {
        let Sca { sph, cyl } = sca;

        if (-20.0..=60.0).contains(&sph) && sph % 0.25 == 0.0 {
            let sca = match cyl {
                Some(Cyl { power, .. }) => {
                    if (1.0..=20.0).contains(&power) && power % 0.25 == 0.0 {
                        sca
                    } else {
                        return Err(IolBoundsError::Cyl(power));
                    }
                }

                None => sca,
            };

            Ok(Self { iol, sca })
        } else {
            Err(IolBoundsError::Se(sph))
        }
    }
}

mod tests {
    use super::*;

    // todo: replace this function with an implementation of Mock
    fn iol() -> Iol {
        Iol {
            model: "ZXTxxx".to_string(),
            name: "Tecnis Symfony".to_string(),
            focus: Focus::Edof,
            toric: true,
        }
    }

    #[test]
    fn makes_new_opiol() {
        let iol = iol();
        let sca = Sca::new(24.25, Some(3.0), Some(12))?;
        let opiol = OpIol::new(iol, sca).unwrap();

        assert_eq!(opiol, OpIol { iol, sca })
    }

    #[test]
    fn out_of_bounds_iol_se_returns_err() {
        // todo: randomize the out of bounds values on all failing tests
        // (Axis, Cyl, Iol, Refraction, Sca, Sia, Target, Va)
        let se = 100.25;
        let iol = iol();
        let sca = Sca::new(se, Some(3.0), Some(12)).unwrap();
        let opiol = OpIol::new(iol, sca);

        assert_eq!(opiol, Err(IolBoundsError::Se(se)))
    }

    #[test]
    fn nonzero_rem_iol_se_returns_err() {
        let se = 10.35;
        let iol = iol();
        let sca = Sca::new(se, Some(3.0), Some(12)).unwrap();
        let opiol = OpIol::new(iol, sca);

        assert_eq!(opiol, Err(IolBoundsError::Se(se)))
    }

    #[test]
    fn out_of_bounds_iol_cyl_power_returns_err() {
        let cyl = 31.0;
        let iol = iol();
        let sca = Sca::new(18.5, Some(cyl), Some(170)).unwrap();
        let opiol = OpIol::new(iol, sca);

        assert_eq!(opiol, Err(IolBoundsError::Cyl(cyl)))
    }

    #[test]
    fn nonzero_rem_iol_cyl_power_returns_err() {
        let cyl = 2.06;
        let iol = iol();
        let sca = Sca::new(28.5, Some(cyl), Some(170)).unwrap();
        let opiol = OpIol::new(iol, sca);

        assert_eq!(opiol, Err(IolBoundsError::Cyl(cyl)))
    }
}
