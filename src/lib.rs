// Do we need a specific type for the Tensor? Perhaps we implement this later...
use std::fmt::Debug;
pub trait TensorType: Default + Debug {
    fn zero() -> Self;
    fn one() -> Self;
}

#[allow(dead_code)]
pub struct Tensor<T> {
    dims: Vec<usize>,
    data: Vec<T>,
}

#[allow(dead_code)]
impl<T> Tensor<T> {
    fn new(dims: Vec<usize>, data: Vec<T>) -> Self  {
        Tensor { dims, data }
    }
}
