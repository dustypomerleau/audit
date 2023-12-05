// todo: break out vision, incision, iol, etc all into separate files
use crate::surgeon::Surgeon;
use serde::Deserialize;
use time::{Date, OffsetDateTime};

// leave off Both, until you have a specific use case for it
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

// We use `best` and `raw` as a more dev-friendly way of saying `bcva` and `ucva`.
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

pub trait Va {} // for common methods on acuity

/// A generic axis between 0 and 179 degrees. The main uses are for the axis of [`RefCyl`] and the
/// meridian of [`Incision`]. In the future, it may also be used for the axis of an implanted [`Iol`].
pub struct Axis(i32);

impl Axis {
    pub fn new(axis: i32) -> Option<Self> {
        if (0..180).contains(&axis) {
            Some(Self(n))
        } else {
            None
        }
    }
}

}

pub struct Refraction {
    sph: f32,
    cyl: Option<Cyl>,
}

// for now, limit this to distance refraction
pub struct OpRefraction {
    before: Refraction,
    after: Refraction,
}

/// The residual postop refraction predicted by your formula of choice.
// At the start, allow only one formula/target.
pub struct Target {
    formula: Option<Formula>,
    se: f32,
    cyl: Option<Cyl>, // confirm which plane the biometry is predicting
}

pub struct Incision {
    meridian: i32,
    sia: Option<f32>,
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

        let target = fc.target_se.and(Some(Target {
            formula: fc.target_formula,
            se: fc.target_se.unwrap(), // won't panic, as `and()` checks the value above
            cyl: fc.target_cyl_power.and(Some(Cyl {
                power: fc.target_cyl_power.unwrap(),
                axis: fc.target_cyl_axis.unwrap(),
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
