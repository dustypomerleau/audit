pub mod biometry;
pub mod case;
pub mod cyl;
pub mod iol;
pub mod refraction;
pub mod sca;
pub mod sia;
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
pub use surgeon::*;
pub use target::*;
pub use va::*;

use crate::bounded::Bounded;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct Year(#[bounded(range = 2025..=2100, default = Utc::now().year() as u32)] u32);
