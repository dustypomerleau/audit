pub struct Sca {
    pub sph: Option<f32>,
    pub cyl: Option<f32>,
    pub axis: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub enum BadSca {
    Sph,
    Cyl,
    Axis,
}
