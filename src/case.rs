use crate::{
    axis::Axis,
    flatcase::FlatCase,
    incision::{Incision, Sia},
    refraction::{OpRefraction, Refraction},
    surgeon::Surgeon,
    target::{Formula, Target, TargetCyl},
    vision::{OpVision, Va},
};
use time::Date;

/// The side of the patient's surgery.
#[derive(Debug, PartialEq)]
pub enum Side {
    Right,
    Left,
}

/// An adverse intraoperative event. It's up to the surgeon to classify, and only one
/// option can be selected. For example, a wrap around split in the rhexis opens the PC, but it's
/// essentially a rhexis complication.
#[derive(Debug, PartialEq)]
pub enum Adverse {
    Rhexis,
    Pc,
    Zonule,
    Other,
}

/// A single surgical case
// for now, leave biometry parameters out - these can be added later with a working system
#[derive(Debug, PartialEq)]
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

        let axis = if let Some(axis) = fc.target_cyl_axis {
            if let Some(axis) = Axis::new(axis) {
                axis
            } // what should optional fields be if they aren't in range coming from fc? throw? In
              // this case I think we make axis None and then TargetCyl::new() will be None.
              // probably remove the second if let and bind axis to Option<i32>, passing it to
              // TargetCyl::new() - in fact, I think that means you don't need this whole let
              // binding, because you can just pass fc.target_cyl_axis directly to the TargetCyl
              // constructor!
        };
        let target = fc.target_se.and(Some(Target {
            formula: fc.target_formula,
            se: fc.target_se.unwrap(), // won't panic, as `and()` checks the value above
            cyl: fc.target_cyl_power.and(Some(TargetCyl {
                power: fc.target_cyl_power.unwrap(),
                axis,
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

#[cfg(test)]
mod tests {
    use super::*;
    use time::Month;

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
            date: Some(Date::from_calendar_date(2022, Month::August, 10).unwrap()),
            site: Some("The Hospital".to_string()),
            incision_meridian: Some(100),
            incision_sia: Some(0.1),
            iol: Some("AMO Symfony".to_string()),
            adverse: Some(Adverse::Rhexis),

            vision_best_before_num: Some(6.0),
            vision_best_before_den: Some(12.0),
            vision_raw_before_num: Some(6.0),
            vision_raw_before_den: Some(24.0),

            vision_best_after_num: Some(6.0),
            vision_best_after_den: Some(5.0),
            vision_raw_after_num: Some(6.0),
            vision_raw_after_den: Some(7.5),

            vision_best_near_before_num: Some(6.0),
            vision_best_near_before_den: Some(9.0),
            vision_raw_near_before_num: None,
            vision_raw_near_before_den: None,

            vision_best_near_after_num: Some(6.0),
            vision_best_near_after_den: Some(6.0),
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
                    power: 2.5,
                    axis: 160,
                }),
            }
            .into(),
        };

        assert_eq!(Case::from(fc), c)
    }
}
