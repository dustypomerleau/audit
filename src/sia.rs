use crate::{axis::Axis, cyl::Cyl};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SiaBoundsError {
    #[error("SIA must be a value between 0 D and 2 D (supplied value: {0})")]
    Sia(f32),
}

#[derive(Debug, PartialEq)]
pub struct Sia(Cyl);

impl TryFrom<Cyl> for Sia {
    type Error = SiaBoundsError;

    fn try_from(cyl: Cyl) -> Result<Self, Self::Error> {
        if (0.0..=2.0).contains(&cyl.power) {
            Ok(Self(cyl))
        } else {
            Err(SiaBoundsError::Sia(cyl.power))
        }
    }
}
