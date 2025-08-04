use crate::{
    bounded::Bounded,
    mock,
    model::{
        Acd, Adverse, AfterVa, Al, Axis, BeforeVa, Biometry, Case, Cct, Email, Focus, Formula, Iol,
        IolSe, K, Kpower, Ks, Lt, Main, OpIol, OpRefraction, OpVa, RefCyl, RefCylPower, RefSph,
        Refraction, Sia, SiaPower, Side, Site, Surgeon, SurgeonCase, SurgeonDefaults, SurgeonSia,
        Target, TargetCyl, TargetCylPower, TargetSe, ToricPower, Va, VaDen, VaNum, Wtw,
    },
};
use chrono::{DateTime, Utc};
use rand::{
    Rng,
    distr::{Alphanumeric, SampleString, StandardUniform},
    rng,
};

bounded!((Prob, f32, 0.0..1.0));

pub trait Mock: Sized {
    fn mock() -> Self;

    /// Mocks an optional value, returning [`None`] at a rate determined by `none_probability`. A
    /// `none_probability` of 0.1 will result in 10% [`None`] values and 90% `Some(Self)` values.
    fn mock_option(none_probability: Prob) -> Option<Self> {
        if rng().random_range(0.0..1.0) < none_probability.inner() {
            None
        } else {
            Some(Self::mock())
        }
    }
}

pub fn random_string(length: usize) -> String {
    let mut rs = Alphanumeric.sample_string(&mut rng(), length);
    rs.shrink_to_fit();

    rs
}

/// A newtype to cap the length of mocked names.
struct Name(String);

impl Mock for Name {
    fn mock() -> Self {
        Self(random_string(8))
    }
}

impl Name {
    fn inner(self) -> String {
        self.0
    }
}

/// Used for mocking of Gel's `ext::auth::Identity`.
struct Identity {
    issuer: String,
    subject: String,
}

impl Mock for Identity {
    fn mock() -> Self {
        Self {
            issuer: "mock issuer".to_string(),
            subject: (0..=20)
                .map(|_| rng().random_range(0..=9).to_string())
                .collect(),
        }
    }
}

impl Mock for Adverse {
    fn mock() -> Self {
        match rng().random_range(0..=3) {
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

impl Mock for DateTime<Utc> {
    fn mock() -> Self {
        let seconds_in_epoch: i64 = rng().random_range(965_442_339..1_753_254_173);

        Self::from_timestamp(seconds_in_epoch, 0).unwrap()
    }
}

impl Mock for Email {
    fn mock() -> Self {
        let prefix = random_string(5);
        let email = format!("{prefix}@mock.com");

        Self::new(&email).unwrap()
    }
}

impl Mock for Focus {
    fn mock() -> Self {
        let index: usize = rng().random_range(0..=2);

        match index {
            0 => Focus::Mono,
            1 => Focus::Edof,
            2 => Focus::Multi,
            _ => unreachable!(),
        }
    }
}

impl Mock for Formula {
    fn mock() -> Self {
        let index: usize = rng().random_range(0..=14);

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
// need to move the iol fetching outside of this function. If you put the Iols into AppState as
// you've suggested below, you can just fetch them from there.
impl Mock for Iol {
    fn mock() -> Self {
        // In the short term, just make random Iol data to get mocks working for tests.
        Self {
            // model: format!("iol-model-{}", random_string(4)),
            // temporarily use a fixed model to get tests working
            model: "sn60wf".to_string(),
            name: Some(format!("iol-name-{}", random_string(4))),
            company: Some(format!("iol-company-{}", random_string(4))),
            focus: Focus::mock(),
            toric: ToricPower::mock_option(Prob::new(0.4).unwrap()),
        }
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
                Axis::mock_option(Prob::new(0.0).unwrap_or_default())
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
            cyl: RefCyl::mock_option(Prob::new(0.2).unwrap_or_default()),
        }
    }
}

impl Mock for Sia {
    fn mock() -> Self {
        Self {
            power: SiaPower::mock(),
            axis: Axis::mock(),
        }
    }
}
impl Mock for Side {
    fn mock() -> Self {
        let sample: bool = rng().sample(StandardUniform);

        match sample {
            true => Side::Right,
            false => Side::Left,
        }
    }
}

impl Mock for Site {
    fn mock() -> Self {
        let index: usize = rng().random_range(0..=3);

        let site = match index {
            0 => "Royal Melbourne Hospital",
            1 => "Royal Victorian Eye and Ear Hospital",
            2 => "Monash Hospital",
            3 => "Alfred Hospital",
            _ => unreachable!(),
        };

        Self {
            name: site.to_string(),
        }
    }
}

impl Mock for Surgeon {
    fn mock() -> Self {
        Self {
            email: Email::mock(),
            terms: DateTime::<Utc>::mock_option(Prob::new(0.01).unwrap_or_default()),
            first_name: Name::mock_option(Prob::new(0.01).unwrap_or_default())
                .map(|name| name.inner()),
            last_name: Name::mock_option(Prob::new(0.01).unwrap_or_default())
                .map(|name| name.inner()),
            defaults: SurgeonDefaults::mock_option(Prob::new(0.01).unwrap_or_default()),
            sia: SurgeonSia::mock(),
        }
    }
}

impl Mock for SurgeonCase {
    fn mock() -> Self {
        Self {
            urn: format!("urn-{}", random_string(8)),
            date: DateTime::<Utc>::mock().date_naive(),
            site: Site::mock_option(Prob::new(0.1).unwrap()),
            case: Case::mock(),
        }
    }
}

impl Mock for SurgeonDefaults {
    fn mock() -> Self {
        Self {
            site: Site::mock_option(Prob::new(0.05).unwrap_or_default()),
            iol: Iol::mock_option(Prob::new(0.01).unwrap_or_default()),
            formula: Formula::mock_option(Prob::new(0.05).unwrap_or_default()),
            custom_constant: rng().random_range(0..1000) == 0,
            main: Main::mock(),
        }
    }
}

impl Mock for SurgeonSia {
    fn mock() -> Self {
        Self {
            right: Sia::mock(),
            left: Sia::mock(),
        }
    }
}

impl Mock for Target {
    fn mock() -> Self {
        let formula = match rng().random_range(0..=9) {
            0 => None,
            _ => Some(Formula::mock()),
        };

        let custom_constant = matches!(rand::rng().random_range(0..=99), 0);

        let cyl = match rng().random_range(0..=19) {
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
        let den: u32 = rng().random_range(400..20_000);

        Self::new(VaNum::new(600).unwrap(), VaDen::new(den).unwrap())
    }
}

pub fn gen_mocks<T: Mock>(n: u32) -> Vec<T> {
    (0..n).map(|_| T::mock()).collect()
}

#[cfg(feature = "ssr")]
mod tests {
    use super::*;
    use crate::error::AppError;

    pub async fn mock_get_iols() -> Result<Vec<Iol>, AppError> {
        let json = gel_tokio::create_client()
            .await?
            .query_json("select Iol { model, name, company, focus, toric };", &())
            .await?
            .to_string();

        Ok(serde_json::from_str::<Vec<Iol>>(json.as_str()).unwrap_or_default())
    }

    #[test]
    fn mocks_iols() {
        let mocks = gen_mocks::<Case>(10);
        // dbg!(mocks);
    }

    #[test]
    fn mocks_refraction() {
        let mocks = gen_mocks::<Refraction>(10);
        // dbg!(mocks);
    }
}
