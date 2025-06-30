use crate::{error::AppError, mock::MockBounded};
#[cfg(feature = "ssr")]
use rand::{
    Rng,
    distr::uniform::{SampleRange, SampleUniform},
};
use std::range::RangeBounds;

pub trait Bounded<Idx>: Sized {
    fn inner(&self) -> Idx;
    fn new(value: Idx) -> Result<Self, AppError>;
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
