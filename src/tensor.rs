//
//  Copyright 2022 Wilhelm Ågren
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a clone of the License at
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
//  Last updated: 22-10-14
//

use crate::Function;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Tensor<'a> {
    dims: Vec<usize>,
    data: Vec<f32>,
    ctx: &'a Function<'a>,
    requires_grad: bool,
}

#[allow(dead_code)]
impl<'a> Tensor<'a> {
    pub fn new(dims: Vec<usize>, data: Vec<f32>) -> Self {
        Self { dims: dims, data: data,
        ctx: Function::null(), requires_grad: false }
    }

    pub fn zeros(dims: Vec<usize>) -> Self {
        let size: usize = dims.iter().product();
        Tensor { dims: dims, data: vec![0.0; size],
        ctx: Function::null(), requires_grad: false }
    }

    pub fn ones(dims: Vec<usize>) -> Self {
        let size: usize = dims.iter().product();
        Tensor { dims: dims, data: vec![1.0; size],
        ctx: Function::null(), requires_grad: false }
    }

    pub fn dims(&'a self) -> &Vec<usize> {
        &self.dims
    }

    pub fn data(&'a self) -> &Vec<f32> {
        &self.data
    }

    pub fn requires_grad(&'a self) -> bool {
        self.requires_grad
    }

    pub fn add(&'a self, other: &'a Tensor) -> Self {
        let ctx: Function = Function::add(self, other);
        match ctx.forward() {
            Ok(tensor) => tensor,
            Err(e) => panic!("Could not perform forward pass, {:?}", e),
        }
    }
}

