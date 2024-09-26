#![allow(dead_code)]

// use crate::case::Case;
// use edgedb_protocol::{model::Uuid, named_args, value::Value};

#[cfg(feature = "ssr")]
mod tests {
    use crate::sia::Sia;
    use edgedb_protocol::{named_args, value::Value, QueryResult};
    use edgedb_tokio::create_client;
    use tokio::test;

    #[tokio::test]
    async fn inserts_sia() {
        let sia = Sia {
            power: 10,
            axis: 100,
        };

        // todo:
        // - [x] test named args on the simple example of Sia
        // - [ ] test getting the client from context
        // - [ ] write a larger query to insert a full Cas
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
