// todo: break out vision, incision, iol, etc all into separate files
use crate::surgeon::Surgeon;
use time::Date;

fn make_powers() -> [f32; 321] {
    let mut powers = [0.0; 321];
    powers[0] = -20.0;
    for i in 1..321 {
        powers[i] = powers[i - 1] + 0.25;
    }
    powers
}

/// A generic representation of all possible powers (diopters), such as those used in IOLs and refractions (-20.0 D to +60.0 D in 0.25 D steps).
const POWERS: [f32; 321] = make_powers(); // -20.0 to +60.0

/// A range of powers (diopters) for the spherical component of a subjective refraction (-20.0 D to
/// +20.0 D in 0.25 D steps).
const REF_SPH_POWERS: &[f32] = &POWERS[0..161]; // -20.0 to +20.0

/// A range of powers (diopters) for the cylinder component of a subjective refraction (-10.0 D to
/// +10.0 D in 0.25 D steps)
// todo: consider whether this should be increased to -20.0 to +20.0
// Why would you limit it if you are going to allow IOL cyl powers +1.0 to +20.0?
const REF_CYL_POWERS: &[f32] = &POWERS[40..121]; // -10.0 to +10.0

/// A range of powers (diopters) for the spherical equivalent labeling of an IOL (-20.0 D to +60.0
/// D in 0.25 D steps).
const IOL_SE_POWERS: &[f32] = &POWERS[..]; // -20.0 to +60.0

/// A range of powers (diopters) for the cylinder power of an IOL (+1.0 to +20.0
/// D in 0.25 D steps, IOL plane).
const IOL_CYL_POWERS: &[f32] = &POWERS[84..161]; // +1.0 to +20.0

/// The side of the patient's surgery.
pub enum Side {
    Right,
    Left,
}

/// An adverse intraoperative event. It's up to the surgeon to classify, and only one
/// option can be selected. For example, a wrap around split in the rhexis opens the PC, but it's
/// essentially a rhexis complication.
pub enum Adverse {
    Rhexis,
    Pc,
    Zonule,
    Other,
}

/// A formula for calculating IOL power from biometry.
// Limited to common thick-lens formulas to start.
// Eventually we will add all the formulas commonly in use.
pub enum Formula {
    Barrett,
    Kane,
}

/// A Snellen-style fractional visual acuity, with numerator and denominator. Units are not
/// specified, but both fields must be in the same unit. This struct is distance-agnostic, and may
/// represent a true distance acuity, or the distance equivalent of a near acuity. For this reason,
/// a [`Vision`] must always be wrapped in a [`VaDistance`] or a [`VaNear`], to distinguish the two
/// cases.
///
/// The type of vision chart is left to the surgeon's discretion, but is presumed to be a Snellen,
/// ETDRS, or similar chart that provides fractional equivalents.
pub struct Vision {
    num: f32,
    den: f32,
}

impl Vision {
    pub fn new(num: f32, den: f32) -> Option<Self> {
        if (0.1..=20.0).contains(&num) && den > 0.0 {
            Some(Self { num, den })
        } else {
            None
        }
    }
}

/// A Snellen-style fractional visual acuity measured at distance.
pub struct VaDistance(Vision);

/// A Snellen-style fractional visual acuity measured at near, and converted to its distance
/// equivalent.
pub struct VaNear(Vision);

/// A collection of preoperative and postoperative visual acuity measurements for a given case. The
/// best-corrected preoperative visual acuity and the best-corrected postoperative visual acuity
/// are mandatory. Near and uncorrected (raw) visual acuities are optional.
pub struct OpVision {
    best_before: VaDistance,
    raw_before: Option<VaDistance>,

    best_after: VaDistance,
    raw_after: Option<VaDistance>,

    best_near_before: Option<VaNear>,
    raw_near_before: Option<VaNear>,

    best_near_after: Option<VaNear>,
    raw_near_after: Option<VaNear>,
}

pub trait Va {} // todo: for common methods on acuity

/// A generic axis between 0 and 179 degrees. The main uses are for the axis of [`RefCyl`] and the
/// meridian of [`Incision`]. In the future, it may also be used for the axis of an implanted [`Iol`].
pub struct Axis(i32);

impl Axis {
    pub fn new(axis: i32) -> Option<Self> {
        if (0..180).contains(&axis) {
            Some(Axis(axis))
        } else {
            None
        }
    }
}

/// The spherical component of a subjective refraction. The type is constrained to values in
/// [`REF_SPH_POWERS`] by the `new()` method on [`Refraction`].
pub struct RefSphPower(f32);

impl RefSphPower {
    pub fn new(value: f32) -> Option<Self> {
        if REF_SPH_POWERS.contains(&value) {
             Some(Self(value))
        } else {
            None
        }
    }
}

/// The cylindrical power component of a subjective refraction. The type is constrained to values in
/// [`REF_CYL_POWERS`] by the `new()` method on [`Refraction`].
pub struct RefCylPower(f32);

impl RefCylPower {
    pub fn new(value: f32) -> Option<Self> {
        if REF_CYL_POWERS.contains(&value) {
             Some(Self(value))
        } else {
            None
        }
    }
}

/// The cylinder component of a subjective refraction, consisting of a cylindrical power in
/// diopters, and an axis in degrees.
pub struct RefCyl {
    power: RefCylPower,
    axis: Axis,
}

impl RefCyl {
    fn new(power: Option<f32>, axis: Option<i32>) -> Option<Self> {
        match (power, axis) {
            (Some(power), Some(axis)) => {
                if let (Some(power), Some(axis)) = (RefCylPower::new(power), Axis::new(axis)) {
                        Some(Self { power, axis })
                } else {
                    None
                }
            }

            (_, _) => {
                None
            }
        }
    }
}

/// A patient's subjective refraction.
pub struct Refraction {
    sph: RefSphPower,
    cyl: Option<RefCyl>,
}

impl Refraction {
    pub fn new(sph: f32, cyl: Option<f32>, axis: Option<i32>) -> Option<Self> {
        match (RefSphPower::new(sph), RefCyl::new(cyl, axis)) {
            (Some(sph), Some(cyl)) => {
                Some(Refraction { sph, cyl: Some(cyl) })
            }

            (Some(sph), None) => {
                Some(Refraction { sph, cyl: None })
            }

            (_, _) => {
                None
            }
        }
    }
}

// for now, limit this to distance refraction
pub struct OpRefraction {
    before: Refraction,
    after: Refraction,
}

pub struct TargetCylPower(f32);

impl TargetCylPower {
    pub fn new(value: f32) -> Option<Self> {
        if (0.0..=6.0).contains(&value) {
             Some(Self(value))
        } else {
            None
        }
    }
}

pub struct TargetCyl {
    power: TargetCylPower,
    axis: Axis,
}

// note: This is the second impl of SomeCyl::new()
// If you do this a third time, perhaps abstract it into a generic.
impl TargetCyl {
    fn new(power: Option<f32>, axis: Option<i32>) -> Option<Self> {
        match (power, axis) {
            (Some(power), Some(axis)) => {
                if let (Some(power), Some(axis)) = (TargetCylPower::new(power), Axis::new(axis)) {
                        Some(Self { power, axis })
                } else {
                    None
                }
            }

            (_, _) => {
                None
            }
        }
    }
}

/// The residual postop refraction predicted by your formula of choice.
// At the start, allow only one formula/target.
pub struct Target {
    formula: Option<Formula>,
    se: f32,
    cyl: Option<TargetCyl>, // todo: confirm which plane the biometry is predicting, IOL or corneal
}

impl Target {
    pub fn new(formula: Option<Formula>, se: f32, cyl: Option<f32>, axis: Option<i32>) -> Option<Self> {
        let cyl = TargetCyl::new(cyl, axis);

        if (-6.0..=2.0).contains(&se) {
            Some(Self { formula, se, cyl, })
        } else {
            None
        }

    }
}

pub struct Sia(f32);

impl Sia {
    pub fn new(sia: f32) -> Option<Self> {
        if (0.0..=2.0).contains(&sia) {
            Some(Self(sia))
        } else {
            None
        }
    }
}

pub struct Incision {
    meridian: Axis,
    sia: Sia,
}

// todo: this mess
// first question to answer is whether to express Incision.sia as None or 0.0 when there isn't one
impl Incision {
    pub fn new(meridian: i32, sia: f32) -> Option<Self> {
        if let (Some(meridian), Some(sia)) = (Axis::new(meridian), Sia::new(sia)) {
            Some(Self { meridian, sia })
        } else {
            None
        }
    }
}

pub struct IolSe(f32);

impl IolSe {
    pub fn new(se: f32) -> Option<Self> {
        if IOL_SE_POWERS.contains(&se) {
            Some(Self(se))
        } else {
            None
        }
    }
}

pub struct IolCyl(f32);

impl IolCyl {
    pub fn new(cyl: f32) -> Option<Self> {
        if IOL_CYL_POWERS.contains(&cyl) {
            Some(Self(cyl))
        } else {
            None
        }
    }
}

pub struct Iol {
    se: IolSe,
    cyl: Option<IolCyl>,
}

/// A single surgical case
// for now, leave biometry parameters out - these can be added later with a working system
pub struct Case {
    surgeon: Surgeon,
    urn: String, // used for the surgeon's reference, not database uniqueness - recommend surgeons have a column to deanonymize
    side: Side,
    target: Option<Target>,
    date: Date, // consider how this will be used: is there any scenario requiring a utc datetime? plan was to have an uploaded datetime, but there isn't any reason to keep this in the struct when you could get it from the DB created_at
    site: Option<String>,
    incision: Option<Incision>,
    iol: Option<String>,
    adverse: Option<Adverse>,
    vision: OpVision,
    refraction: OpRefraction,
}

impl From<FlatCase> for Case {
    fn from(fc: FlatCase) -> Self {
        let surgeon = Surgeon {
            email: fc.surgeon_email.expect("surgeon to have an email"),
            first_name: fc.surgeon_first_name,
            last_name: fc.surgeon_last_name,
            site: fc.surgeon_site,
        };

        let urn = fc.urn.expect("case to have a URN");
        let side = fc.side.expect("case to have a Side");

        // todo: fix this, you can't just unwrap axis - do you maybe want to work on that cyl trait before proceeding?
        let axis = Axis::new(fc.target_cyl_axis.unwrap()).unwrap();
        let target = fc.target_se.and(Some(Target {
            formula: fc.target_formula,
            se: fc.target_se.unwrap(), // won't panic, as `and()` checks the value above
            cyl: fc.target_cyl_power.and(Some(TargetCyl {
                power: fc.target_cyl_power.unwrap(),
                axis
            })),
        }));

        let date = fc.date.expect("case to have a Date");
        let site = fc.site;

        let incision = fc.incision_meridian.and(Some(Incision {
            meridian: fc.incision_meridian.unwrap(),
            sia: fc.incision_sia,
        }));

        let iol = fc.iol;
        let adverse = fc.adverse;

        let vision = OpVision {
            best_before: VaDistance(Vision {
                num: fc
                    .vision_best_before_num
                    .expect("vision to have a numerator"),
                den: fc
                    .vision_best_before_den
                    .expect("vision to have a denominator"),
            }),
            raw_before: fc.vision_raw_before_den.and(Some(VaDistance(Vision {
                num: fc.vision_raw_before_num.unwrap(),
                den: fc.vision_raw_before_den.unwrap(),
            }))),

            best_after: VaDistance(Vision {
                num: fc
                    .vision_best_after_num
                    .expect("vision to have a numerator"),
                den: fc
                    .vision_best_after_den
                    .expect("vision to have a denominator"),
            }),
            raw_after: fc.vision_raw_after_den.and(Some(VaDistance(Vision {
                num: fc.vision_raw_after_num.unwrap(),
                den: fc.vision_raw_after_den.unwrap(),
            }))),

            best_near_before: fc.vision_best_near_before_den.and(Some(VaNear(Vision {
                num: fc.vision_best_near_before_num.unwrap(),
                den: fc.vision_best_near_before_den.unwrap(),
            }))),
            raw_near_before: fc.vision_raw_near_before_den.and(Some(VaNear(Vision {
                num: fc.vision_raw_near_before_num.unwrap(),
                den: fc.vision_raw_near_before_den.unwrap(),
            }))),

            best_near_after: fc.vision_best_near_after_den.and(Some(VaNear(Vision {
                num: fc.vision_best_near_after_num.unwrap(),
                den: fc.vision_best_near_after_den.unwrap(),
            }))),
            raw_near_after: fc.vision_raw_near_after_den.and(Some(VaNear(Vision {
                num: fc.vision_raw_near_after_num.unwrap(),
                den: fc.vision_raw_near_after_den.unwrap(),
            }))),
        };

        let refraction = OpRefraction {
            before: Refraction {
                sph: fc
                    .refraction_before_sph
                    .expect("refraction to have a spherical component"),
                cyl: fc.refraction_before_cyl_power.and(Some(Cyl {
                    power: fc.refraction_before_cyl_power.unwrap(),
                    axis: fc.refraction_before_cyl_axis.unwrap(),
                })),
            },
            after: Refraction {
                sph: fc
                    .refraction_after_sph
                    .expect("refraction to have a spherical component"),
                cyl: fc.refraction_after_cyl_power.and(Some(Cyl {
                    power: fc.refraction_after_cyl_power.unwrap(),
                    axis: fc.refraction_after_cyl_axis.unwrap(),
                })),
            },
        };

        Case {
            surgeon,
            urn,
            side,
            target,
            date,
            site,
            incision,
            iol,
            adverse,
            vision,
            refraction,
        }
    }
}

/// A flattened version of the Case struct for DB queries.
// All fields are optional, to allow using the same struct for any DB query on Case.
pub struct FlatCase {
    surgeon_email: Option<String>,
    surgeon_first_name: Option<String>,
    surgeon_last_name: Option<String>,
    surgeon_site: Option<String>,
    urn: Option<String>,
    side: Option<Side>,
    target_formula: Option<Formula>,
    target_se: Option<f32>,
    target_cyl_power: Option<f32>,
    target_cyl_axis: Option<i32>,
    date: Option<Date>,
    site: Option<String>,
    incision_meridian: Option<i32>,
    incision_sia: Option<f32>,
    iol: Option<String>,
    adverse: Option<Adverse>,

    vision_best_before_num: Option<f32>,
    vision_best_before_den: Option<f32>,
    vision_raw_before_num: Option<f32>,
    vision_raw_before_den: Option<f32>,

    vision_best_after_num: Option<f32>,
    vision_best_after_den: Option<f32>,
    vision_raw_after_num: Option<f32>,
    vision_raw_after_den: Option<f32>,

    vision_best_near_before_num: Option<f32>,
    vision_best_near_before_den: Option<f32>,
    vision_raw_near_before_num: Option<f32>,
    vision_raw_near_before_den: Option<f32>,

    vision_best_near_after_num: Option<f32>,
    vision_best_near_after_den: Option<f32>,
    vision_raw_near_after_num: Option<f32>,
    vision_raw_near_after_den: Option<f32>,

    refraction_before_sph: Option<f32>,
    refraction_before_cyl_power: Option<f32>,
    refraction_before_cyl_axis: Option<i32>,

    refraction_after_sph: Option<f32>,
    refraction_after_cyl_power: Option<f32>,
    refraction_after_cyl_axis: Option<i32>,
}

#[cfg(test)]
mod tests {
    use time::Date;

    use crate::surgeon::Surgeon;

    use super::{Adverse, FlatCase, Side};

    #[test]
    fn case_implements_from_flatcase() {
        let fc = FlatCase {
            surgeon_email: Some("test@test.com".to_string()),
            surgeon_first_name: Some("test first".to_string()),
            surgeon_last_name: Some("test last".to_string()),
            surgeon_site: None,
            urn: Some("AB700693".to_string()),
            side: Some(Side::Right),
            target_formula: None,
            target_se: Some(-0.1),
            target_cyl_power: Some(2.5),
            target_cyl_axis: Some(160),
            date: Some(Date::from_calendar_date(2022, 8, 10)),
            site: Some("The Hospital".to_string()),
            incision_meridian: Some(100),
            incision_sia: Some(0.1),
            iol: Some("AMO Symfony".to_string()),
            adverse: Some(Adverse::Rhexis),

            vision_best_before_num: Some(6),
            vision_best_before_den: Some(12),
            vision_raw_before_num: Some(6),
            vision_raw_before_den: Some(24),

            vision_best_after_num: Some(6),
            vision_best_after_den: Some(5),
            vision_raw_after_num: Some(6),
            vision_raw_after_den: Some(7.5),

            vision_best_near_before_num: Some(6),
            vision_best_near_before_den: Some(9),
            vision_raw_near_before_num: None,
            vision_raw_near_before_den: None,

            vision_best_near_after_num: Some(6),
            vision_best_near_after_den: Some(6),
            vision_raw_near_after_num: None,
            vision_raw_near_after_den: None,

            refraction_before_sph: Some(-5.25),
            refraction_before_cyl_power: Some(1.5),
            refraction_before_cyl_axis: Some(10),

            refraction_after_sph: Some(0.25),
            refraction_after_cyl_power: Some(-0.5),
            refraction_after_cyl_axis: Some(12),
        };

        let c = Case {
            surgeon: Surgeon {
                email: "test@test.com".to_string(),
                first_name: Some("test first".to_string()),
                last_name: Some("test last".to_string()),
                site: None,
            },

            urn: "AB700693".to_string(),
            side: Side::Right,

            target: Target {
                formula: None,
                se: Some(-0.1),
                cyl: Some(TargetCyl {
                    power: 
                    axis: 160
                })
            }
            .into(),
        };

        assert_eq!(Case::from(fc), c)
    }
}
