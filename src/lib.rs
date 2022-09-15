use std::fmt::Debug;

// Here we define the type of the tensor as a trait, and defines
// default and required functions of all Tensor structs.
pub trait TensorType: Default + Debug {
    fn zero() -> Self;
    fn one() -> Self;
}

pub struct Tensor {
    dims: Vec<u64>,
    // Here we want the data, but how should we store it?
}
