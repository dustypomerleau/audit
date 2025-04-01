use std::range::RangeBounds;

pub trait Bounded<Idx>: Sized {
    fn range() -> impl RangeBounds<Idx>;
    fn inner(&self) -> Idx;
}
