use crate::{
    bounded::Bounded,
    model::{RefCylPower, RefSph, TargetCylPower, TargetSe},
};

/// A marker trait for measurements with the following properties:
///
/// 1. They are taken in the spectacle plane
/// 2. They are stored in integer form, as (diopters * 100)
pub trait Spectacle: Bounded<Idx: Into<f64>> {}

impl Spectacle for RefCylPower {}
impl Spectacle for RefSph {}
impl Spectacle for TargetCylPower {}
impl Spectacle for TargetSe {}

/// Types that can be vertexed from their plane to the corneal plane (from either the spectacle or
/// IOL planes).
pub trait VertexK {
    /// Returns the power in diopters at the corneal plane. Be sure to convert to diopters for any
    /// value stored in the DB as (diopters * 100).
    ///
    /// In general:
    ///
    /// Power(corneal plane) =
    ///     Power(spectacle plane) / (1 - (Power(spectacle plane)(Vertex distance in meters)))
    ///
    /// For a default vertex distance of 13 mm we get:
    ///
    /// Kpower = Spower / (1 - (Spower * 0.013))
    fn vertex(&self) -> f64;
}

impl<T: Spectacle> VertexK for T {
    fn vertex(&self) -> f64 {
        let spectacle = self.inner().into() / 100.0;

        spectacle / (1.0 - (spectacle * 0.013))
    }
}
