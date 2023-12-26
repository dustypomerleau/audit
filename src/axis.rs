/// A generic axis between 0 and 179 degrees. The main uses are for the axis of [`RefCyl`] and the
/// meridian of [`Incision`]. In the future, it may also be used for the axis of an implanted [`Iol`].
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
