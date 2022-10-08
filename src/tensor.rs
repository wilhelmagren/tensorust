//
//  Copyright 2022 Wilhelm Ågren
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//
//  File created: 22-10-03
//  Last updated: 22-10-03
//

use crate::Function;

#[allow(dead_code)]
pub struct Tensor {
    dims: Vec<usize>,
    data: Vec<f32>,
    ctx: Function,
    requires_grad: bool,
}

#[allow(dead_code)]
impl Tensor {
    pub fn new(dims: Vec<usize>, data: Vec<f32>) -> Tensor {
        Tensor { dims: dims, data: data, ctx: Function::empty(), requires_grad: false }
    }

    pub fn zeros(dims: Vec<usize>) -> Tensor {
        let size: usize = dims.iter().product();
        Tensor { dims: dims, data: vec![0.0; size] , ctx: Function::empty(), requires_grad: false }
    }

    pub fn ones(dims: Vec<usize>) -> Tensor {
        let size: usize = dims.iter().product();
        Tensor { dims: dims, data: vec![1.0; size] , ctx: Function::empty(), requires_grad: false }
    }

    pub fn dims(&self) -> &Vec<usize> {
        &self.dims
    }

    pub fn data(&self) -> &Vec<f32> {
        &self.data
    }
}

// Look into more idiomatic way of implementing traits for Tensor.
use std::ops::Add;
impl Add for Tensor {
    type Output = Self;
    fn add(self, other: Tensor) -> Tensor {
        let data: Vec<f32> = self.data.iter().zip(other.data.iter())
            .map(|(&u, &v)| u + v)
            .collect();
        Tensor { dims: self.dims, data: data , ctx: Function::empty(), requires_grad: false }
    }
}

