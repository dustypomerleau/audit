/// A generic axis between 0° and 179°. This is used for the axis of [`Cyl`](crate::cyl::Cyl)
/// in [`Refraction`](crate::refraction::Refraction), [`Target`](crate::target::Target),
/// [`Iol`](crate::iol::Iol), and [`Sia`](crate::sia::Sia).
#[derive(Clone, Copy, Debug, PartialEq)]
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
