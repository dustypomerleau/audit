use crate::sca::Sca;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Unchecked;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Checked;

pub trait BoundsCheck: Sca {
    type Error;
    type CheckedOutput;

    fn check(self) -> Result<Self::CheckedOutput, Self::Error>;
}
