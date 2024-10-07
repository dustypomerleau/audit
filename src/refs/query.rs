use crate::{
    bounds_check::{BoundsCheck, Unchecked},
    case::{Adverse, Case, Side},
    cyl::Cyl,
    iol::{Focus, Iol, OpIol},
    refraction::{OpRefraction, Refraction},
    sia::Sia,
    target::{Constant, Formula, Target},
    va::{AfterVa, BeforeVa, OpVa, Va},
};
use chrono::NaiveDate;
use edgedb_protocol::{
    named_args,
    value::{EnumValue, Value},
    QueryResult,
};
use edgedb_tokio::create_client;
use std::{fmt, marker::PhantomData, sync::Arc};

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_case() -> Case {
        Case {
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
                    sph: -25,
                    cyl: Some(Cyl {
                        power: -25,
                        axis: 60,
                    }),
                    bounds: PhantomData,
                }
                .check()
                .unwrap(),
            },
        }
    }

    // #[tokio::test]
    // async fn inserts_sia() {
    //     let sia = Sia {
    //         power: 10,
    //         axis: 100,
    //     };
    //
    //     let case = sample_case();
    //
    //     // target_constant_value
    //     // target_constant_formula
    //     // target_se
    //     // target_cyl_power
    //     // target_cyl_axis
    //     // todo: you need logic so that the named args are only used if Case::target.is_some()
    //     // try wrapping each Option in ValueOpt and just passing it as is
    //
    //     // todo:
    //     // - [x] test named args on the simple example of Sia
    //     // - [ ] write a larger query to insert a full Cas: (you don't need to assign all
    //     // the intermediate `with` statements, just use dot notation off the incoming
    //     // `Case`, as you did with `Sia` below).
    //     //
    //     // Within components, the client will be provided via:
    //     // let client = expect_context::<Client>();
    //     let client = create_client().await.expect("DB client to be created");
    //     let query = "insert Sia { power := <int32>$sia_power, axis := <int32>$sia_axis };";
    //
    //     let target = case.target.unwrap_or(Value::Nothing);
    //
    //     let Target {
    //         constant, se, cyl, ..
    //     } = target;
    //
    //     let constant = match constant {
    //         Some(constant) => constant,
    //         None => Value::Nothing,
    //     };
    //
    //     let args = named_args! {
    //         "urn" => Value::Str(case.urn),
    //         "side" => Value::Enum(EnumValue::from(case.side.to_string().as_str())),
    //         "sia_power" => Value::Int32(sia.power),
    //         "sia_axis" => Value::Int32(sia.axis)
    //     };
    //
    //     let res: Vec<Value> = client.query(query, &args).await.expect("query to succeed");
    //
    //     println!("{res:?}");
    // }
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
