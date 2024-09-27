#![allow(dead_code)]

// use crate::case::Case;
// use edgedb_protocol::{model::Uuid, named_args, value::Value};

#[cfg(test)]
mod tests {
    use crate::{
        bounds_check::{BoundsCheck, Checked, Unchecked},
        case::{Adverse, Case, Side},
        cyl::Cyl,
        iol::{Focus, Iol, OpIol},
        refraction::{OpRefraction, Refraction},
        sia::Sia,
        surgeon::{Email, Surgeon, SurgeonSia},
        target::{Constant, Formula, Target},
        va::{AfterVa, BeforeVa, OpVa, Va},
    };
    use chrono::NaiveDate;
    use edgedb_protocol::{named_args, value::Value, QueryResult};
    use edgedb_tokio::create_client;
    use std::marker::PhantomData;
    use tokio::test;

    fn sample_case() -> Case {
        Case {
            surgeon: Surgeon {
                email: Email::new("email@email.com").unwrap(),
                first_name: Some("john".to_string()),
                last_name: Some("smith".to_string()),
                sites: Some(vec![
                    "Royal Melbourne Hospital".to_string(),
                    "Manningham Private Hospital".to_string(),
                ]),
                sia: Some(SurgeonSia {
                    right: Sia {
                        power: 10,
                        axis: 100,
                    },
                    left: Sia {
                        power: 10,
                        axis: 100,
                    },
                }),
            },

            urn: "abc123".to_string(),
            side: Side::Right,

            target: Some(
                Target::<Unchecked> {
                    constant: Some(Constant {
                        value: 11936,
                        formula: Formula::Kane,
                    }),
                    se: -12,
                    cyl: Some(Cyl {
                        power: 15,
                        axis: 90,
                    }),
                    bounds: PhantomData,
                }
                .check()
                .unwrap(),
            ),

            date: NaiveDate::from_ymd_opt(2022, 3, 15).unwrap(),
            site: Some("RMH".to_string()),
            sia: None,

            iol: Some(
                OpIol::<Unchecked> {
                    iol: Iol {
                        model: "zxr100".to_string(),
                        name: "Symfony".to_string(),
                        company: "Johnson and Johnson".to_string(),
                        focus: Focus::Edof,
                        toric: true,
                    },
                    se: 2400,
                    cyl: Some(Cyl {
                        power: 100,
                        axis: 90,
                    }),
                    bounds: PhantomData,
                }
                .check()
                .unwrap(),
            ),

            adverse: Some(Adverse::Pc),

            va: OpVa {
                before: BeforeVa {
                    best: Va {
                        num: 600,
                        den: 1200,
                    },
                    raw: None,
                },
                after: AfterVa {
                    best: None,
                    raw: Va { num: 600, den: 500 },
                },
            },

            refraction: OpRefraction {
                before: Refraction::<Unchecked> {
                    sph: 300,
                    cyl: Some(Cyl {
                        power: -125,
                        axis: 45,
                    }),
                    bounds: PhantomData,
                }
                .check()
                .unwrap(),
                after: Refraction::<Unchecked> {
                    sph: -025,
                    cyl: Some(Cyl {
                        power: -025,
                        axis: 60,
                    }),
                    bounds: PhantomData,
                }
                .check()
                .unwrap(),
            },
        }
    }

    #[tokio::test]
    async fn inserts_sia() {
        let sia = Sia {
            power: 10,
            axis: 100,
        };

        // todo:
        // - [x] test named args on the simple example of Sia
        // - [ ] write a larger query to insert a full Cas
        //
        // Within components, the client will be provided via:
        // let client = expect_context::<Client>();
        let client = create_client().await.expect("DB client to be created");
        let query = "insert Sia { power := <int32>$sia_power, axis := <int32>$sia_axis };";

        let args = named_args! {
            "sia_power" => Value::Int32(sia.power),
            "sia_axis" => Value::Int32(sia.axis)
        };

        let res: Vec<Value> = client.query(query, &args).await.expect("query to succeed");

        println!("{res:?}");

        ()
    }
}

// pub fn insert_case(case: Case) -> Uuid {
//     let Case {
//         urn,
//         side,
//         target,
//         date,
//         site,
//         sia,
//         iol,
//         adverse,
//         va,
//         refraction,
//         ..
//     } = case;
//
//     let (target_constant, target_se, target_cyl_power, target_cyl_axis) =
//         if let Some(Target {
//             constant, se, cyl, ..
//         }) = target
//         {
//             let constant = if let Some(constant) = constant {
//                 Value::Int32(constant)
//             } else {
//                 Value::Nothing
//             };
//
//             // if Cyl is None, we don't want to pass the values to named args?...
//             //
//             let
//         };
//
//     // value side of this macro must be `impl Into<ValueOpt>`.
//     // most of these are implemented already:
//     // https://docs.rs/edgedb-protocol/latest/edgedb_protocol/value/enum.Value.html
//     named_args! {
//         "urn" => urn,
//         "side" => side,
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//         "adverse" =>
//     }
// }
