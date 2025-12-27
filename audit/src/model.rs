pub mod biometry;
pub mod case;
pub mod cyl;
pub mod iol;
pub mod refraction;
pub mod sca;
pub mod sia;
pub mod split_option;
pub mod surgeon;
pub mod target;
pub mod va;

use audit_macro::RangeBounded;
pub use biometry::*;
pub use case::*;
use chrono::Datelike;
use chrono::Utc;
pub use cyl::*;
pub use iol::*;
pub use refraction::*;
pub use sca::*;
use serde::Deserialize;
use serde::Serialize;
pub use sia::*;
pub use split_option::*;
pub use surgeon::*;
pub use target::*;
pub use va::*;

use crate::bounded::Bounded;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize)]
#[bounded(range = 2025..=2100, default = Utc::now().year())]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Year(i32);
