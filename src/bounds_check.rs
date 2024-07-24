use crate::sca::Sca;
use serde::{Deserialize, Serialize};

/// A zero-sized type representing a value that has not been bounds-checked.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Unchecked;

/// A zero-sized type representing a bounds-checked value.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Checked;

/// Implementing [`BoundsCheck`] for a [`Sca`] type allows transitioning between unbounded and
/// bounds-checked states.
pub trait BoundsCheck: Sca {
    type Error;
    type CheckedOutput;

    fn check(self) -> Result<Self::CheckedOutput, Self::Error>;
}
