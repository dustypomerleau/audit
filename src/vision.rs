/// A Snellen-style fractional visual acuity, with numerator and denominator. Units are not
/// specified, but both fields must be in the same unit. This struct is distance-agnostic, and may
/// represent a true distance acuity, or the distance equivalent of a near acuity. For this reason,
/// a [`Vision`] must always be wrapped in a [`VaDistance`] or a [`VaNear`], to distinguish the two
/// cases.
///
/// The type of vision chart is left to the surgeon's discretion, but is presumed to be a Snellen,
/// ETDRS, or similar chart that provides fractional equivalents.
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub struct VaDistance(pub Vision);

/// A Snellen-style fractional visual acuity measured at near, and converted to its distance
/// equivalent.
#[derive(Debug, PartialEq)]
pub struct VaNear(pub Vision);

/// A collection of preoperative and postoperative visual acuity measurements for a given case. The
/// best-corrected preoperative visual acuity and the best-corrected postoperative visual acuity
/// are mandatory. Near and uncorrected (raw) visual acuities are optional.
#[derive(Debug, PartialEq)]
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

pub trait Va {} // todo: for common methods on acuity
