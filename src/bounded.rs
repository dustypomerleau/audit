use crate::mock::MockBounded;
#[cfg(feature = "ssr")]
use rand::{
    Rng,
    distr::uniform::{SampleRange, SampleUniform},
};
use serde::{Deserialize, Serialize};
use std::range::RangeBounds;
use thiserror::Error;

/// A wrapper for any type of bounds error on numeric types.
#[derive(Clone, Debug, Deserialize, Error, PartialEq, Serialize)]
#[error("the value is out of bounds {0:?}")]
pub struct BoundsError(pub String);

pub trait Bounded<Idx>: Sized {
    fn inner(&self) -> Idx;
    fn new(value: Idx) -> Result<Self, BoundsError>;
    fn range() -> impl RangeBounds<Idx>;
}

#[cfg(feature = "ssr")]
impl<Idx, T> MockBounded<Idx> for T
where
    Idx: SampleUniform,
    T: Bounded<Idx>,
    T::range(..): SampleRange<Idx>,
{
    fn mock_bounded() -> Self {
        let random_inner = rand::rng().random_range(Self::range());

        // We can unwrap here, because `random_inner` is selected from the bounded range for T.
        Self::new(random_inner).unwrap()
    }
}
