#[cfg(feature = "ssr")] use std::range::RangeBounds;

#[cfg(feature = "ssr")] use rand::distr::uniform::SampleRange;
#[cfg(feature = "ssr")] use rand::distr::uniform::SampleUniform;

use crate::error::AppError;

/// A numeric type with bounds that can be described by a [`Range`](std::range::Range).
pub trait Bounded: Sized {
    /// The numeric type of the [`RangeBounds`]. Due to the default implementation of
    /// [`Mock`](crate::mock::Mock) on these types, we require that [`Bounded::Idx`]:
    /// [`SampleUniform`].
    #[cfg(feature = "ssr")]
    type Idx: SampleUniform;

    /// The numeric type of the [`RangeBounds`].
    #[cfg(not(feature = "ssr"))]
    type Idx;

    /// Return the inner numeric value of the type.
    fn inner(&self) -> Self::Idx;

    /// Create a new [`Bounded`] instance.
    fn new(value: Self::Idx) -> Result<Self, AppError>;

    /// Return the [`Range`](std::range::Range) that bounds the type.
    #[cfg(feature = "ssr")]
    fn range() -> impl RangeBounds<Self::Idx> + SampleRange<Self::Idx>;

    /// If a [`Rem`] value is required, return the value passed to the `%` predicate in the `new`
    /// constructor.
    fn rem() -> Option<Self::Idx>;
}
