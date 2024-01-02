use crate::axis::Axis;

#[derive(Debug, PartialEq)]
pub struct Sia(f32);

impl Sia {
    pub fn new(sia: f32) -> Option<Self> {
        if (0.0..=2.0).contains(&sia) {
            Some(Self(sia))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Incision {
    meridian: Axis,
    sia: Sia,
}

impl Incision {
    // If no sia is given by the surgeon (meaning FlatCase contains sia: None), you'll
    // have to call this with 0.0.
    pub fn new(meridian: i32, sia: f32) -> Option<Self> {
        if let (Some(meridian), Some(sia)) = (Axis::new(meridian), Sia::new(sia)) {
            Some(Self { meridian, sia })
        } else {
            None
        }
    }
}
