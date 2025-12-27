/// Several structs in this crate serve to pair items, so that they are either both present
/// or both absent. For example, [`Cyl`](crate::model::Cyl) types must contain both a power
/// and an axis. For performance reasons, in the database these types are often represented as
/// adjacent columns, rather than a separate table with foreign key. This trait is typically
/// implemented on [`Option<T>`], where `T` is a struct containing a pair that should either be
/// split into a tuple of its component parts or a tuple of [`None`]s. Pattern-matching the returned
/// tuple makes it easy to bind variables for database insertion without checking if the optional
/// value is present first.
pub trait SplitOption {
    type A;
    type B;

    fn split_option(&self) -> (Option<Self::A>, Option<Self::B>);
}
