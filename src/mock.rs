use crate::{
    bounded::Bounded,
    components::get_iols,
    model::{
        Acd, Adverse, AfterVa, Al, Axis, BeforeVa, Biometry, Case, Cct, Formula, Iol, IolSe, K,
        Kpower, Ks, Lt, Main, OpIol, OpRefraction, OpVa, RefCyl, RefCylPower, RefSph, Refraction,
        Sia, SiaPower, Side, Target, TargetCyl, TargetCylPower, TargetSe, Va, VaDen, VaNum, Wtw,
    },
};
use rand::{
    Rng,
    distr::{
        StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
};
use serde::{Deserialize, Serialize};

bounded!((Prob, f32, 0.0..1.0));

pub trait Mock: Sized {
    fn mock() -> Self;

    /// Mocks an optional value, returning [`None`] at a rate determined by `none_probability`. A
    /// `none_probability` of 0.1 will result in 10% [`None`] values and 90% `Some(Self)` values.
    fn mock_option(none_probability: Prob) -> Option<Self> {
        let n = (none_probability.inner() * 100.0) as u32;

        if rand::rng().random_range(0..100) < n {
            None
        } else {
            Some(Self::mock())
        }
    }
}

impl<T> Mock for T
where
    T: Bounded,
    T::Idx: SampleUniform,
    T::range(..): SampleRange<T::Idx>,
{
    fn mock() -> Self {
        let random_inner = rand::rng().random_range(Self::range());

        // Safe unwrap: `random_inner` is selected from the bounded range for T.
        Self::new(random_inner).unwrap()
    }
}

impl Mock for Adverse {
    fn mock() -> Self {
        match rand::rng().random_range(0..=3) {
            0 => Adverse::Rhexis,
            1 => Adverse::Pc,
            2 => Adverse::Zonule,
            3 => Adverse::Other,
            _ => unreachable!(),
        }
    }
}

impl Mock for AfterVa {
    fn mock() -> Self {
        Self {
            best: Va::mock_option(Prob::new(0.2).unwrap_or_default()),
            raw: Va::mock(),
        }
    }
}

impl Mock for BeforeVa {
    fn mock() -> Self {
        Self {
            best: Va::mock(),
            raw: Va::mock_option(Prob::new(0.2).unwrap_or_default()),
        }
    }
}

impl Mock for Biometry {
    fn mock() -> Self {
        Biometry {
            al: Al::mock(),
            ks: Ks::new(
                K::new(Kpower::mock(), Axis::mock()),
                K::new(Kpower::mock(), Axis::mock()),
            ),
            acd: Acd::mock(),
            lt: Lt::mock(),
            cct: Cct::mock_option(Prob::new(0.05).unwrap_or_default()),
            wtw: Wtw::mock_option(Prob::new(0.05).unwrap_or_default()),
        }
    }
}

impl Mock for Case {
    fn mock() -> Self {
        Self {
            side: Side::mock(),
            biometry: Biometry::mock(),
            target: Target::mock(),
            main: Main::mock(),
            sia: Sia::new(SiaPower::mock(), Axis::mock()),
            iol: OpIol::mock(),
            adverse: Adverse::mock_option(Prob::new(0.99).unwrap_or_default()),
            va: OpVa::mock(),
            refraction: OpRefraction::mock(),
        }
    }
}

impl Mock for Formula {
    fn mock() -> Self {
        let index: usize = rand::rng().random_range(0..=14);

        match index {
            0 => Formula::AscrsKrs,
            1 => Formula::Barrett,
            2 => Formula::BarrettTrueK,
            3 => Formula::Evo,
            4 => Formula::Haigis,
            5 => Formula::HaigisL,
            6 => Formula::HillRbf,
            7 => Formula::HofferQ,
            8 => Formula::Holladay1,
            9 => Formula::Holladay2,
            10 => Formula::Kane,
            11 => Formula::Okulix,
            12 => Formula::Olsen,
            13 => Formula::SrkT,
            14 => Formula::Other,
            _ => unreachable!(),
        }
    }
}

// todo: if you are iterating through many mocks, you don't want to create a runtime for each. You
// need to move the iol fetching outside of this function.
impl Mock for Iol {
    fn mock() -> Self {
        let iols = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(get_iols())
            .unwrap();

        let index = rand::rng().random_range(0..iols.len());

        iols[index].clone()
    }
}

impl Mock for OpIol {
    fn mock() -> Self {
        let iol = Iol::mock();
        let toric = iol.toric.is_some();

        Self {
            iol,
            se: IolSe::mock(),
            axis: if toric {
                Axis::mock_option(Prob::new(1.0).unwrap_or_default())
            } else {
                None
            },
        }
    }
}

impl Mock for OpRefraction {
    fn mock() -> Self {
        Self {
            before: Refraction::mock(),
            after: Refraction::mock(),
        }
    }
}

impl Mock for OpVa {
    fn mock() -> Self {
        Self {
            before: BeforeVa::mock(),
            after: AfterVa::mock(),
        }
    }
}

impl Mock for RefCyl {
    fn mock() -> Self {
        Self {
            power: RefCylPower::mock(),
            axis: Axis::mock(),
        }
    }
}

impl Mock for Refraction {
    fn mock() -> Self {
        Self {
            sph: RefSph::mock(),
            cyl: RefCyl::mock_option(Prob::new(0.8).unwrap_or_default()),
        }
    }
}

impl Mock for Side {
    fn mock() -> Self {
        let sample: bool = rand::rng().sample(StandardUniform);

        match sample {
            true => Side::Right,
            false => Side::Left,
        }
    }
}

impl Mock for Target {
    fn mock() -> Self {
        let formula = match rand::rng().random_range(0..=9) {
            0 => None,
            _ => Some(Formula::mock()),
        };

        let custom_constant = matches!(rand::rng().random_range(0..=99), 0);

        let cyl = match rand::rng().random_range(0..=19) {
            0 => None,
            _ => Some(TargetCyl::new(TargetCylPower::mock(), Axis::mock())),
        };

        Self {
            formula,
            custom_constant,
            se: TargetSe::mock(),
            cyl,
        }
    }
}

impl Mock for Va {
    fn mock() -> Self {
        let den: u32 = rand::rng().random_range(400..20_000);

        Self::new(VaNum::new(600).unwrap(), VaDen::new(den).unwrap())
    }
}
