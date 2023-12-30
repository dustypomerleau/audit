use crate::{
    axis::Axis,
    powers::{REF_CYL_POWERS, REF_SPH_POWERS},
    sca::{BadSca, Sca},
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
// todo: use an enum here, with the same pattern you used in TargetCyl, and change the new function
// to take non-optional fields. For now, probably also remove sph_bounds() and cyl_bounds() on
// Refraction and just inline those checks, as there isn't any other location you would use those
// functions at the moment.
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

#[derive(Debug, PartialEq)]
pub enum Refraction {
    OutOfBounds(BadSca),
    Sph(RefSphPower),
    Cyl { sph: RefSphPower, cyl: RefCyl },
}

impl Refraction {
    pub fn new(sph: f32, cyl: Option<f32>, axis: Option<i32>) -> Self {
        if Self::sph_bounds(sph) {
            match (cyl, axis) {
                (Some(cyl), Some(axis)) => {
                    if Self::cyl_bounds(cyl) {
                        if let Some(axis) = Axis::new(axis) {
                            Self::Cyl {
                                sph: RefSphPower(sph),
                                cyl: RefCyl {
                                    power: RefCylPower(cyl),
                                    axis,
                                },
                            }
                        } else {
                            Self::OutOfBounds(BadSca::Axis)
                        }
                    } else {
                        Self::OutOfBounds(BadSca::Cyl)
                    }
                }

                _ => Self::Sph(RefSphPower(sph)),
            }
        } else {
            Self::OutOfBounds(BadSca::Sph)
        }
    }

    fn sph_bounds(f: f32) -> bool {
        if REF_SPH_POWERS.contains(&f) {
            true
        } else {
            false
        }
    }

    fn cyl_bounds(f: f32) -> bool {
        if REF_CYL_POWERS.contains(&f) {
            true
        } else {
            false
        }
    }
}

impl From<Sca> for Refraction {
    fn from(s: Sca) -> Self {
        let Sca { sph, cyl, axis } = s;

        if let Some(sph) = sph {
            Refraction::new(sph, cyl, axis)
        } else {
            Self::OutOfBounds(BadSca::Sph)
        }
    }
}

// for now, limit this to distance refraction
#[derive(Debug, PartialEq)]
pub struct OpRefraction {
    before: Refraction,
    after: Refraction,
}
