use crate::axis::Axis;

/// An agnostic cylinder type. The acceptable bounds of [`power`](Cyl::power) depend on the
/// type of power being represented. Since the acceptable values of [`axis`](Cyl::axis) are always
/// the same, we insist upon an [`Axis`](crate::axis::Axis) to constrain it.
#[derive(Debug, PartialEq)]
pub struct Cyl {
    pub power: f32,
    pub axis: Axis,
}

/// Representation of missing cylinder components for use by error types.
#[derive(Debug, PartialEq)]
pub enum CylPair {
    Power,
    Axis,
}
