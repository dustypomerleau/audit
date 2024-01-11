/// A distance wrapper to ensure that distance and near values ([`Refraction`](crate::refraction::Refraction), [`Va`](crate::va::Va)) are not confused.
#[derive(Debug, PartialEq)]
pub struct Distance<T>(T);

/// A near wrapper to ensure that distance and near values ([`Refraction`](crate::refraction::Refraction), [`Va`](crate::va::Va)) are not confused.
#[derive(Debug, PartialEq)]
pub struct Near<T>(T);
