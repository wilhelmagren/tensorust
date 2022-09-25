// Do we need a specific type for the Tensor? Perhaps we implement this later...
use std::fmt::Debug;
pub trait TensorType: Default + Debug {
    fn zero() -> Self;
    fn one() -> Self;
}

#[allow(dead_code)]
pub struct Tensor {
    dims: Vec<usize>,
    data: Vec<f32>
}

#[allow(dead_code)]
impl Tensor {
    pub fn new(dims: Vec<usize>, data: Vec<f32>) -> Tensor {
        Tensor { dims, data }
    }

    pub fn zeros(dims: Vec<usize>) -> Tensor {
        let size: usize = dims.iter().product();
        Tensor { dims, data: vec![0.0; size] }
    }

    pub fn ones(dims: Vec<usize>) -> Tensor {
        let size: usize = dims.iter().product();
        Tensor { dims, data: vec![1.0; size] }
    }

    pub fn dims(&self) -> &Vec<usize> {
        &self.dims
    }

    pub fn data(&self) -> &Vec<f32> {
        &self.data
    }
}

