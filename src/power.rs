// https://stackoverflow.com/questions/54048500/convert-literal-to-associated-type-in-generic-struct-implementation
// https://stackoverflow.com/questions/54504026/how-do-i-provide-an-implementation-of-a-generic-struct-in-rust

use crate::cyl::Cyl;

pub struct Power {
    pub sph: f32,
    pub cyl: Option<Cyl>,
}
