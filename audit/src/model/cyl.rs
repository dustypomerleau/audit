use audit_macro::RangeBounded;
use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
use crate::model::Kpower;
use crate::model::RefCylPower;
use crate::model::SiaPower;
use crate::model::SplitOption;
use crate::model::TargetCylPower;

pub trait CylPower {}
impl CylPower for Kpower {}
impl CylPower for RefCylPower {}
impl CylPower for SiaPower {}
impl CylPower for TargetCylPower {}
impl CylPower for i32 {}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize,
)]
#[bounded(range = 0..=179)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Axis(i32);

pub trait Cyl {
    type Power: CylPower;

    fn power(&self) -> Self::Power;
    fn axis(&self) -> Axis;
}

impl<T: Cyl> SplitOption for Option<T> {
    type A = T::Power;
    type B = Axis;

    fn split_option(&self) -> (Option<Self::A>, Option<Self::B>) {
        if let Some(cyl) = self {
            (Some(cyl.power()), Some(cyl.axis()))
        } else {
            (None, None)
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct RawCyl {
    pub power: i32,
    pub axis: Axis,
}

impl Cyl for RawCyl {
    type Power = i32;

    fn power(&self) -> Self::Power { self.power }

    fn axis(&self) -> Axis { self.axis }
}

impl RawCyl {
    pub fn new(power: i32, axis: Axis) -> Self { Self { power, axis } }
}
