use crate::error::AppError;
#[cfg(feature = "ssr")] use rand::distr::uniform::{SampleRange, SampleUniform};
#[cfg(feature = "ssr")] use std::range::RangeBounds;

pub trait Bounded: Sized {
    #[cfg(feature = "ssr")]
    type Idx: SampleUniform;

    #[cfg(not(feature = "ssr"))]
    type Idx;

    fn inner(&self) -> Self::Idx;
    fn new(value: Self::Idx) -> Result<Self, AppError>;

    #[cfg(feature = "ssr")]
    fn range() -> impl RangeBounds<Self::Idx> + SampleRange<Self::Idx>;
}
