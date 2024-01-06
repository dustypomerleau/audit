/// A generic axis between 0° and 179°. The main uses are for the axes of
/// [`RefCyl`](crate::refraction::RefCyl), [`TargetCyl`](crate::target::TargetCyl), [`IolCyl`](crate::iol::IolCyl), and the
/// meridian of [`Incision`](crate::incision::Incision::meridian).
#[derive(Debug, PartialEq)]
pub struct Axis(i32);

impl Axis {
    pub fn new(axis: i32) -> Option<Self> {
        if (0..180).contains(&axis) {
            Some(Axis(axis))
        } else {
            None
        }
    }
}
