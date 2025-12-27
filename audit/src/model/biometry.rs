use audit_macro::RangeBounded;
use serde::Deserialize;
use serde::Serialize;

use crate::bounded::Bounded;
use crate::error::AppError;
use crate::model::Axis;
use crate::model::Cyl;

// TODO: use a more evidence-based approach to choosing biometry defaults.
//
// NOTE: In this location, and many others, the values of the wrapped type cannot be negative, but
// we use [`i32`] to ensure that the inner type is compatible with Postgres arrays (see
// [`sqlx::sqlx_postgres::PgHasArrayType`]). It is possible to opt out of this with
// `#[sqlx(trasparent, no_pg_array)]` if needed, but there is no upside here, since we are
// controlling bounds with the constructor
//
/// An [`i32`] wrapper representing the depth of the anterior chamber in dm.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize)]
#[bounded(range = 0..=600, default = 350, mock_range = 250..=450)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Acd(i32);

/// An [`i32`] wrapper representing the axial length in dm.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize)]
#[bounded(range = 1200..=3800, default = 2400, mock_range = 2200..=2800)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Al(i32);

/// An [`i32`] wrapper representing the central corneal thickness in micrometers.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize)]
#[bounded(range = 350..=650, default = 550, mock_range = 450..=600)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Cct(i32);

/// An [`i32`] wrapper representing the corneal curvature in (diopters * 100).
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize)]
#[bounded(range = 3000..=6500, default = 4400, mock_range = 3800..=4700)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Kpower(i32);

/// An [`i32`] wrapper representing the lens thickness in dm.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize)]
#[bounded(range = 200..=800, default = 450, mock_range = 350..=550)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Lt(i32);

/// An [`i32`] wrapper representing the white-to-white distance in dm.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, RangeBounded, Serialize)]
#[bounded(range = 800..=1400, default = 1200, mock_range = 1000..=1300)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(transparent))]
pub struct Wtw(i32);

/// The corneal curvature in a single meridian.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct K {
    pub power: Kpower,
    pub axis: Axis,
}

impl Cyl for K {
    type Power = Kpower;

    fn power(&self) -> Self::Power { self.power }

    fn axis(&self) -> Axis { self.axis }
}

impl K {
    pub fn new(power: Kpower, axis: Axis) -> Self { Self { power, axis } }
}

// Safety: These fields are private to enforce the invariants that flat <= steep and flat.axis =
// (steep.axis + 90°).
/// A set of biometric Ks, 90° apart.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Ks {
    flat: K,
    steep: K,
}

impl Ks {
    // TODO: this function should enforce the invariant that the axes are 90° apart.
    pub fn new(k1: K, k2: K) -> Result<Self, AppError> {
        if !(k1.axis.inner() - k2.axis.inner() == 90 || k2.axis.inner() - k1.axis.inner() == 90) {
            let (ka1, ka2) = (k1.axis, k2.axis);

            return Err(AppError::Bounds(format!(
                "the axes of a `biometry::Ks` should be 90° apart, but the given values were k1.axis: {ka1}, k2.axis: {ka2}"
            )));
        }

        let ks = if k1.power.inner() <= k2.power.inner() {
            Self {
                flat: k1,
                steep: k2,
            }
        } else {
            Self {
                flat: k2,
                steep: k1,
            }
        };

        Ok(ks)
    }

    pub fn flat_power(&self) -> Kpower { self.flat.power }

    pub fn steep_power(&self) -> Kpower { self.steep.power }

    pub fn cyl(&self) -> i32 { self.steep_power().inner() - self.flat_power().inner() }

    pub fn flat_axis(&self) -> Axis { self.flat.axis }

    pub fn steep_axis(&self) -> Axis { self.steep.axis }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Biometry {
    pub al: Al,
    pub ks: Ks,
    pub acd: Acd,
    pub lt: Lt,
    pub cct: Option<Cct>,
    pub wtw: Option<Wtw>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assigns_ks_correctly() {
        let ks = Ks::new(
            K::new(Kpower::new(4230).unwrap(), Axis::new(100).unwrap()),
            K::new(Kpower::new(4025).unwrap(), Axis::new(10).unwrap()),
        )
        .unwrap();

        assert!(ks.flat_power() < ks.steep_power());
    }
}
