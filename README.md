# Rust tensor library
![Build](https://github.com/willeagren/tensorust/actions/workflows/rust-build.yml/badge.svg)
![Tests](https://github.com/willeagren/tensorust/actions/workflows/rust-tests.yml/badge.svg)


Tensorust is a general purpose tensor library written in Rust. It features no C/C++ bindings, which are otherwise commonly used in tensor libraries, e.g. Tensorflow and PyTorch. Tensorust is pure Rust. The heart of the library is dynamic building of a tensor DAG and performs automatic gradient calculation. Currently only supports Float16 operations.

# License
All code is written under an Apache-2.0 styled license, please see [LICENSE](https://github.com/willeagren/tensorust/blob/main/LICENSE) for more information.

