use crate::{bounded::Bounded, range_bounded};
use serde::{Deserialize, Serialize};

pub trait CylPower {}
impl CylPower for i32 {}
impl CylPower for u32 {}

range_bounded!((Axis, u32, 0..=179));

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
    fn power(&self) -> i32 {
        self.power
    }

    fn axis(&self) -> Axis {
        self.axis
    }
}

impl RawCyl {
    pub fn new(power: i32, axis: Axis) -> Self {
        Self { power, axis }
    }
}
