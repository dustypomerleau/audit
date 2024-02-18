use serde::{Deserialize, Serialize};

/// A bounds-checked generic axis between 0° and 179°. This is used for the axis of
/// [`Cyl`](crate::cyl::Cyl) in [`Refraction`](crate::refraction::Refraction),
/// [`Target`](crate::target::Target), [`Iol`](crate::iol::Iol), and [`Sia`](crate::sia::Sia).
///
/// The purist would prefer using `meridian` rather than `axis` for [`Sia`](crate::sia::Sia) and
/// biometric Ks, but on balance I've decided that the cognitive overhead of using both terms in the
/// code is higher than the cognitive overhead of knowing when `axis` actually refers to a
/// meridian.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Axis(pub i32);

impl Axis {
    /// Create a new [`Axis`] with bounds checking.
    pub fn new(axis: i32) -> Option<Self> {
        if (0..180).contains(&axis) {
            Some(Axis(axis))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_new_axis() {
        let axis = Axis::new(90);
        assert_eq!(axis, Some(Axis(90)))
    }

    #[test]
    fn negative_axis_returns_none() {
        let axis = Axis::new(-1);
        assert_eq!(axis, None)
    }

    #[test]
    fn out_of_bounds_axis_returns_none() {
        let axis = Axis::new(300);
        assert_eq!(axis, None)
    }
}
