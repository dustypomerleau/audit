pub trait Cyl
where
    Self: Sized,
{
    /// Checks the power and axis of the given cylinder type, ensuring that they are within
    /// appropriate bounds.
    fn new_with_bounds(power: Option<f32>, axis: Option<i32>) -> Option<Self>;
}
