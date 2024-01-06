/// A generic type representing any cylinder measurement. Used mostly to guide error types.
#[derive(Debug, PartialEq)]
pub(crate) enum Cyl {
    Power,
    Axis,
}
