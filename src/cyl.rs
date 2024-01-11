use crate::axis::Axis;

/// A parent-agnostic cylinder type. The acceptable bounds of [`power`](Cyl::power) depend on the
/// type of power being represented. Since the acceptable values of [`axis`](Cyl::axis) are always
/// the same, we insist upon an [`Axis`](crate::axis::Axis) to constrain it.
#[derive(Debug, PartialEq)]
pub struct Cyl {
    pub power: f32,
    pub axis: Axis,
}

#[derive(Debug, PartialEq)]
pub enum CylPair {
    Power,
    Axis,
}
