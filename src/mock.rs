use crate::bounded::Bounded;

pub trait Mock {
    fn mock() -> Self;
}

pub trait MockBounded<Idx>: Bounded<Idx> {
    fn mock_bounded() -> Self;
}
