// todo: I think you need whatever the modern alternative was to lazy_static! or something
// research how to do this
fn make_powers() -> [f32; 321] {
    let mut powers = [0.0; 321];
    powers[0] = -20.0;
    for i in 1..321 {
        powers[i] = powers[i - 1] + 0.25;
    }
    powers
}

/// A generic representation of all possible powers (diopters), such as those used in IOLs and refractions (-20.0 D to +60.0 D in 0.25 D steps).
pub const POWERS: [f32; 321] = make_powers(); // -20.0 to +60.0

/// A range of powers (diopters) for the spherical component of a subjective refraction (-20.0 D to
/// +20.0 D in 0.25 D steps).
pub const REF_SPH_POWERS: &[f32] = &POWERS[0..161]; // -20.0 to +20.0

/// A range of powers (diopters) for the cylinder component of a subjective refraction (-10.0 D to
/// +10.0 D in 0.25 D steps)
// todo: consider whether this should be increased to -20.0 to +20.0
// Why would you limit it if you are going to allow IOL cyl powers +1.0 to +20.0?
pub const REF_CYL_POWERS: &[f32] = &POWERS[40..121]; // -10.0 to +10.0

/// A range of powers (diopters) for the spherical equivalent labeling of an IOL (-20.0 D to +60.0
/// D in 0.25 D steps).
pub const IOL_SE_POWERS: &[f32] = &POWERS[..]; // -20.0 to +60.0

/// A range of powers (diopters) for the cylinder power of an IOL (+1.0 to +20.0
/// D in 0.25 D steps, IOL plane).
pub const IOL_CYL_POWERS: &[f32] = &POWERS[84..161]; // +1.0 to +20.0
