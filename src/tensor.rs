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

// This is extremely ugly, but it works at least.
// Look into more idiomatic way of implementing traits for Tensor.
use std::ops::Add;
impl Add for Tensor {
    type Output = Self;
    fn add(self, other: Tensor) -> Tensor {
        let data: Vec<f32> = self.data.iter().zip(other.data.iter())
            .map(|(&u, &v)| u + v)
            .collect();
        Tensor { dims: self.dims, data }
    }
}

