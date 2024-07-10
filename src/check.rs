use crate::sca::Sca;

#[derive(Clone, Debug, PartialEq)]
pub struct Unchecked;

#[derive(Clone, Debug, PartialEq)]
pub struct Checked;

pub trait BoundsCheck: Sca {
    type Error;
    type Output;

    fn check(self) -> Result<Self::Output, Self::Error>;
}
