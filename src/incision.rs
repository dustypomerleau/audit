use crate::axis::Axis;
use thiserror::Error;

#[derive(Debug, Error)]
enum IncisionBoundsError {
    #[error(
        "incision meridian must be an integer value between 0° and 179° (supplied value: {0})"
    )]
    Meridian(i32),
    #[error("SIA must be a value between 0 D and 2 D (supplied value: {0})")]
    Sia(f32),
}

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
    pub fn new(meridian: i32, sia: f32) -> Result<Self, IncisionBoundsError> {
        if let Some(meridian) = Axis::new(meridian) {
            if let Some(sia) = Sia::new(sia) {
                Ok(Self { meridian, sia })
            } else {
                Err(IncisionBoundsError::Sia(sia))
            }
        } else {
            Err(IncisionBoundsError::Meridian(meridian))
        }
    }
}
