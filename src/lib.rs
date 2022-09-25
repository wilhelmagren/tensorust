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
    fn new(dims: Vec<usize>, data: Vec<f32>) -> Tensor {
        Tensor { dims, data }
    }

    fn zeros(dims: Vec<usize>) -> Tensor {
        let size: usize = dims.iter().product();
        Tensor { dims, data: vec![0.0; size] }
    }

    fn ones(dims: Vec<usize>) -> Tensor {
        let size: usize = dims.iter().product();
        Tensor { dims, data: vec![1.0; size] }
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t: Tensor = Tensor::new(vec![2, 3], vec![2.0; 6]);
        assert_eq!(t.dims, vec![2, 3]);
        assert_eq!(t.data, vec![2.0; 6]);
    }
}
