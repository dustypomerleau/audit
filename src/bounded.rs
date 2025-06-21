use serde::{Deserialize, Serialize};
use std::range::RangeBounds;
use thiserror::Error;

/// A wrapper for any type of bounds error on numeric types.
#[derive(Clone, Debug, Deserialize, Error, PartialEq, Serialize)]
#[error("the value is out of bounds {0:?}")]
pub struct BoundsError(pub String);

pub trait Bounded<Idx>: Sized {
    fn range() -> impl RangeBounds<Idx>;
    fn inner(&self) -> Idx;
}
