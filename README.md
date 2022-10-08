# Rust tensor library
![Build](https://github.com/willeagren/tensorust/actions/workflows/rust-build.yml/badge.svg)
![Tests](https://github.com/willeagren/tensorust/actions/workflows/rust-tests.yml/badge.svg)


Tensorust is a general purpose tensor library written in Rust. It features no C/C++ bindings, which are otherwise commonly used in tensor libraries, e.g. Tensorflow and PyTorch. Tensorust is pure Rust. The heart of the library is dynamic building of a tensor DAG and performs automatic gradient calculation. 

The current status of the library is in early v0.1 stages; experiments are being conducted to verify data layout in memory for optimal performance. The only datatype supported for the Tensor struct as of writing this is float32, but making it a generic type is work in progress.

The currently implemented Tensor operations are:
- add

Operations to implement are:
- sub
- mul
- dot
- matmul
- conv2d
- pool2d
- activation functions (tanh, relu, sigmoid, ...)


## Example
```rust
use tensorust::{Size, Tensor};

// Create two Tensors of the same size; one with only zeros and 
// the other with only ones. Add them and get resulting Tensor.
fn main() -> Result<()> {
  let a: Tensor = Tensor::zeros(Size::from_vec(vec![128, 64]));
  let b: Tensor = Tensor::ones(Size::from_vec(vec![128, 64]));
  
  let c: Tensor = a + b;
  // An alternative way to add the Tensors is to explicitly call
  // let c: Tensor = a.add(b);
  
  Ok(())
}
```


# License
All code is written under an Apache-2.0 styled license, please see [LICENSE](https://github.com/willeagren/tensorust/blob/main/LICENSE) for more information.

