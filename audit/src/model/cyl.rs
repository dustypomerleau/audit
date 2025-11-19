use audit_macro::RangeBounded;
use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;

pub trait CylPower {}
impl CylPower for i32 {}
impl CylPower for u32 {}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, RangeBounded, Serialize)]
pub struct Axis(#[bounded(range = 0..=179)] u32);

pub trait Cyl<T>
where T: CylPower
{
    fn power(&self) -> T;
    fn axis(&self) -> Axis;
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct RawCyl {
    pub power: i32,
    pub axis: Axis,
}

impl Cyl<i32> for RawCyl {
    fn power(&self) -> i32 { self.power }

    fn axis(&self) -> Axis { self.axis }
}

impl RawCyl {
    pub fn new(power: i32, axis: Axis) -> Self { Self { power, axis } }
}
