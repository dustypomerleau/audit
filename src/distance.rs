use serde::{Deserialize, Serialize};

pub trait Distance<T> {}

/// A far wrapper to ensure that far and near values
/// ([`Refraction`](crate::refraction::Refraction), [`Va`](crate::va::Va)) are not confused.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Far<T>(pub T);

impl<T> Distance<T> for Far<T> {}

impl<T> From<T> for Far<T> {
    fn from(value: T) -> Self { Self(value) }
}

/// A near wrapper to ensure that far and near values
/// ([`Refraction`](crate::refraction::Refraction), [`Va`](crate::va::Va)) are not confused.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Near<T>(pub T);

impl<T> Distance<T> for Near<T> {}

impl<T> From<T> for Near<T> {
    fn from(value: T) -> Self { Self(value) }
}
