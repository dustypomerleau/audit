// use crate::{
//     bounds_check::Checked,
//     case::Adverse,
//     cyl::Cyl,
//     iol::{Iol, OpIol},
//     refraction::OpRefraction,
//     va::OpVa,
// };
// #[cfg(feature = "ssr")] use edgedb_derive::Queryable;
// use serde::{Deserialize, Serialize};
//
// // This whole thing is a fool's errand...
// // just get the case out into a generic struct, impl TryFrom the generic flat struct for
// PlotCase, // and use regular types you've already defined in the PlotCase
// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[cfg_attr(feature = "ssr", derive(Queryable))]
// pub struct PlotIolCyl {
//     power: i32,
//     axis: i32,
// }
//
// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[cfg_attr(feature = "ssr", derive(Queryable))]
// pub struct PlotIol {
//     pub iol: Iol,
//     pub se: i32,
//     pub cyl: Option<PlotIolCyl>,
// }
//
// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[cfg_attr(feature = "ssr", derive(Queryable))]
// pub struct PlotCase {
//     year: i32,
//     iol: Option<PlotIol>,
//     adverse: Option<Adverse>,
//     va: OpVa,
//     refraction: PlotRefraction,
// }
//
// // todo: remove this
// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// struct PlotRefraction;
