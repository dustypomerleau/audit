use audit_macro::RangeBounded;
use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
use crate::model::SplitOption;

// Choosing not to use NonZeroU32 for VaDen, because it has a slightly different interface than all
// our other bounded types.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize)]
#[bounded( range = 1..=i32::MAX, default = 600, mock_range = 500..=6000)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct VaDen(i32);

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize)]
#[bounded(range = 0..=2000, default = 600, mock_range = 600..=600)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct VaNum(i32);

/// A Snellen-style fractional visual acuity, with numerator and denominator. Units are not
/// specified, but both fields must be in the same unit.  
///
/// The type of vision chart is left to the surgeon's discretion, but is presumed to be a Snellen,
/// ETDRS, or similar chart that provides fractional equivalents.
///
/// Values are represented as `((entered float) * 100) as i32`, and stored in the DB as `i32` with
/// constraints. This makes the representation consistent with [`Cyl`](crate::cyl::Cyl),
/// [`Iol`](crate::iol::Iol), [`Refraction`](crate::refraction::Refraction), and
/// [`Target`](crate::target::Target).
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Va {
    pub num: VaNum,
    pub den: VaDen,
}

impl SplitOption for Option<Va> {
    type A = VaNum;
    type B = VaDen;

    fn split_option(&self) -> (Option<Self::A>, Option<Self::B>) {
        if let Some(Va { num, den }) = *self {
            (Some(num), Some(den))
        } else {
            (None, None)
        }
    }
}

impl Va {
    /// Creates a new [`Va`] with bounds checking.
    pub fn new(num: VaNum, den: VaDen) -> Self { Self { num, den } }

    pub fn num(&self) -> VaNum { self.num }

    pub fn den(&self) -> VaDen { self.den }
}

/// A collection of visual acuities from before surgery. We use separate structs for [`BeforeVa`]
/// and [`AfterVa`], because we enforce different mandatory fields for the two situations.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BeforeVa {
    pub best: Va,
    pub raw: Option<Va>,
}

/// A collection of visual acuities from after surgery. We use separate structs for [`BeforeVa`]
/// and [`AfterVa`], because we enforce different mandatory fields for the two situations.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AfterVa {
    pub best: Option<Va>,
    pub raw: Va,
}

/// The visual acuity sets from before and after a particular [`Case`](crate::case::Case).
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OpVa {
    pub before: BeforeVa,
    pub after: AfterVa,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_new_va() {
        assert!(VaNum::new(600).is_ok());
        assert!(VaDen::new(750).is_ok());
    }

    #[test]
    fn out_of_bounds_va_numerator_returns_err() {
        assert!(VaNum::new(2120).is_err());
    }

    #[test]
    fn zero_va_denominator_returns_err() {
        assert!(VaDen::new(0).is_err());
    }
}
