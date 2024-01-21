use crate::{refraction::Refraction, va::Va};

/// A far wrapper to ensure that far and near values
/// ([`Refraction`](crate::refraction::Refraction), [`Va`](crate::va::Va)) are not confused.
#[derive(Debug, PartialEq)]
pub struct Far<T>(pub T);

impl From<Va> for Far<Va> {
    fn from(va: Va) -> Self { Self(va) }
}

impl From<Refraction> for Far<Refraction> {
    fn from(refraction: Refraction) -> Self { Self(refraction) }
}

/// A near wrapper to ensure that far and near values
/// ([`Refraction`](crate::refraction::Refraction), [`Va`](crate::va::Va)) are not confused.
#[derive(Debug, PartialEq)]
pub struct Near<T>(pub T);

impl From<Va> for Near<Va> {
    fn from(va: Va) -> Self { Self(va) }
}

impl From<Refraction> for Near<Refraction> {
    fn from(refraction: Refraction) -> Self { Self(refraction) }
}

