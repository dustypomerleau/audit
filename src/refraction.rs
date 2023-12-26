use crate::{
    case::Axis,
    powers::{REF_CYL_POWERS, REF_SPH_POWERS},
};

/// The spherical component of a subjective refraction. The type is constrained to values in
/// [`REF_SPH_POWERS`] by the `new()` method on [`Refraction`].
#[derive(Debug, PartialEq)]
pub struct RefSphPower(f32);

impl RefSphPower {
    pub fn new(value: f32) -> Option<Self> {
        if REF_SPH_POWERS.contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

/// The cylindrical power component of a subjective refraction. The type is constrained to values in
/// [`REF_CYL_POWERS`] by the `new()` method on [`Refraction`].
#[derive(Debug, PartialEq)]
pub struct RefCylPower(f32);

impl RefCylPower {
    pub fn new(value: f32) -> Option<Self> {
        if REF_CYL_POWERS.contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }
}

/// The cylinder component of a subjective refraction, consisting of a cylindrical power in
/// diopters, and an axis in degrees.
#[derive(Debug, PartialEq)]
pub struct RefCyl {
    power: RefCylPower,
    axis: Axis,
}

impl RefCyl {
    fn new(power: Option<f32>, axis: Option<i32>) -> Option<Self> {
        match (power, axis) {
            (Some(power), Some(axis)) => {
                if let (Some(power), Some(axis)) = (RefCylPower::new(power), Axis::new(axis)) {
                    Some(Self { power, axis })
                } else {
                    None
                }
            }

            (_, _) => None,
        }
    }
}

/// A patient's subjective refraction.
#[derive(Debug, PartialEq)]
pub struct Refraction {
    sph: RefSphPower,
    cyl: Option<RefCyl>,
}

impl Refraction {
    pub fn new(sph: f32, cyl: Option<f32>, axis: Option<i32>) -> Option<Self> {
        match (RefSphPower::new(sph), RefCyl::new(cyl, axis)) {
            (Some(sph), Some(cyl)) => Some(Refraction {
                sph,
                cyl: Some(cyl),
            }),

            (Some(sph), None) => Some(Refraction { sph, cyl: None }),

            (_, _) => None,
        }
    }
}

// for now, limit this to distance refraction
#[derive(Debug, PartialEq)]
pub struct OpRefraction {
    before: Refraction,
    after: Refraction,
}
