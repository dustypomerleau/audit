use crate::surgeon::Surgeon;
use serde::Deserialize;
use time::{Date, OffsetDateTime};

// leave off Both, until you have a specific use case for it
pub enum Side {
    Right,
    Left,
}

/// Represents an adverse intraoperative event. It's up to the surgeon to classify, and only one
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
pub enum Formula {
    Barrett,
    Kane,
}

pub struct Vision {
    num: i32,
    den: i32,
}

// make sure that distance and near acuities are not interchangeable
pub struct VaDistance(Vision);
pub struct VaNear(Vision);

// We use `best` and `raw` as a more dev-friendly way of saying `bcva` and `ucva`.
pub struct OpVision {
    best_before: VaDistance,
    best_after: VaDistance,
    raw_before: Option<VaDistance>,
    raw_after: Option<VaDistance>,
    best_near_before: Option<VaNear>,
    best_near_after: Option<VaNear>,
    raw_near_before: Option<VaNear>,
    raw_near_after: Option<VaNear>,
}

pub trait Va {} // for common methods on acuity

pub struct Cyl {
    power: f32,
    axis: i32,
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
    sia: Option<i32>,
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
        Case {
            surgeon: Surgeon {
                email: fc.surgeon_email.expect("surgeon to have an email"),
                first_name: fc.surgeon_first_name,
                last_name: fc.surgeon_last_name,
                site: fc.surgeon_site,
            },
            urn: fc.urn.expect("to have a URN"),
            side: fc.side.expect("to have a Side"),
            target: fc.target_se.and(Some(Target {
                formula: fc.target_formula,
                se: fc.target_se.unwrap(), // won't panic, as `and()` checks the value above
                cyl: fc.target_cyl_power.and(Some(Cyl {
                    power: fc.target_cyl_power.unwrap(),
                    axis: fc.target_cyl_axis.unwrap(),
                })),
            })),
            date: fc.date.expect("surgery to have a Date"),
            site: fc.site,
            incision: fc.incision_meridian.and(Some(Incision {
                meridian: fc.incision_meridian.unwrap(),
                sia: fc.incision_sia,
            })),
            iol: fc.iol,
            adverse: fc.adverse,
            vision: OpVision {
                best_before: VaDistance(Vision {
                    num: fc
                        .vision_best_before_num
                        .expect("vision to have a numerator"),
                    den: fc
                        .vision_best_before_den
                        .expect("vision to have a denominator"),
                }),
                best_after: VaDistance(Vision {
                    num: fc
                        .vision_best_after_num
                        .expect("vision to have a numerator"),
                    den: fc
                        .vision_best_after_den
                        .expect("vision to have a denominator"),
                }),
                raw_before: fc.vision_raw_before_den.and(Some(VaDistance(Vision {
                    num: fc.vision_raw_before_num.unwrap(),
                    den: fc.vision_raw_before_den.unwrap(),
                }))),
                raw_after: fc.vision_raw_after_den.and(Some(VaDistance(Vision {
                    num: fc.vision_raw_after_num.unwrap(),
                    den: fc.vision_raw_after_den.unwrap(),
                }))),
                best_near_before: fc.vision_best_near_before_den.and(Some(VaNear(Vision {
                    num: fc.vision_best_near_before_num.unwrap(),
                    den: fc.vision_best_near_before_den.unwrap(),
                }))),
                best_near_after: fc.vision_best_near_after_den.and(Some(VaNear(Vision {
                    num: fc.vision_best_near_after_num.unwrap(),
                    den: fc.vision_best_near_after_den.unwrap(),
                }))),
                raw_near_before: fc.vision_raw_near_before_den.and(Some(VaNear(Vision {
                    num: fc.vision_raw_near_before_num.unwrap(),
                    den: fc.vision_raw_near_before_den.unwrap(),
                }))),
                raw_near_after: fc.vision_raw_near_after_den.and(Some(VaNear(Vision {
                    num: fc.vision_raw_near_after_num.unwrap(),
                    den: fc.vision_raw_near_after_den.unwrap(),
                }))),
            },
            refraction: OpRefraction {
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
            },
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
    incision_sia: Option<i32>,
    iol: Option<String>,
    adverse: Option<Adverse>,
    vision_best_before_num: Option<i32>,
    vision_best_before_den: Option<i32>,
    vision_best_after_num: Option<i32>,
    vision_best_after_den: Option<i32>,
    vision_raw_before_num: Option<i32>,
    vision_raw_before_den: Option<i32>,
    vision_raw_after_num: Option<i32>,
    vision_raw_after_den: Option<i32>,
    vision_best_near_before_num: Option<i32>,
    vision_best_near_before_den: Option<i32>,
    vision_best_near_after_num: Option<i32>,
    vision_best_near_after_den: Option<i32>,
    vision_raw_near_before_num: Option<i32>,
    vision_raw_near_before_den: Option<i32>,
    vision_raw_near_after_num: Option<i32>,
    vision_raw_near_after_den: Option<i32>,
    refraction_before_sph: Option<f32>,
    refraction_before_cyl_power: Option<f32>,
    refraction_before_cyl_axis: Option<i32>,
    refraction_after_sph: Option<f32>,
    refraction_after_cyl_power: Option<f32>,
    refraction_after_cyl_axis: Option<i32>,
}
