use crate::refraction::Refraction;

/// A distance wrapper to ensure that distance and near values
/// ([`Refraction`](crate::refraction::Refraction), [`Va`](crate::va::Va)) are not confused.
#[derive(Debug, PartialEq)]
pub struct Distance<T>(T);

impl From<Va> for Distance<Va> {
    fn from(va: Va) -> Self { Self(va) }
}

impl From<Refraction> for Distance<Refraction> {
    fn from(refraction: Refraction) -> Self { Self(refraction) }
}

/// A near wrapper to ensure that distance and near values
/// ([`Refraction`](crate::refraction::Refraction), [`Va`](crate::va::Va)) are not confused.
#[derive(Debug, PartialEq)]
pub struct Near<T>(T);

impl From<Va> for Near<Va> {
    fn from(va: Va) -> Self { Self(va) }
}

impl From<Refraction> for Near<Refraction> {
    fn from(refraction: Refraction) -> Self { Self(refraction) }
}
