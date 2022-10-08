//
//  cloneright 2022 Wilhelm Ågren
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
//  Last updated: 22-10-08
//

use crate::Size;
use crate::Function;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Tensor {
    dims: Size,
    data: Vec<f32>,
    ctx: Function,
    requires_grad: bool,
}

#[allow(dead_code)]
impl Tensor {
    pub fn new(dims: Size, data: Vec<f32>) -> Tensor {
        Tensor { dims: dims, data: data, ctx: Function::empty(), requires_grad: false }
    }

    pub fn zeros(dims: Size) -> Tensor {
        let size: usize = dims.dims().iter().product();
        Tensor { dims: dims, data: vec![0.0; size] , ctx: Function::empty(), requires_grad: false }
    }

    pub fn ones(dims: Size) -> Tensor {
        let size: usize = dims.dims().iter().product();
        Tensor { dims: dims, data: vec![1.0; size] , ctx: Function::empty(), requires_grad: false }
    }

    pub fn dims(&self) -> &Size {
        &self.dims
    }

    pub fn data(&self) -> &Vec<f32> {
        &self.data
    }

    pub fn ctx(&self) -> &Function {
        &self.ctx
    }

    pub fn requires_grad(&self) -> bool {
        self.requires_grad
    }

    pub fn add(&self, other: &Tensor) -> Self {
        let dims: Size = self.dims.clone();
        let data: Vec<f32> = self.data.iter()
            .zip(other.data.iter())
            .map(|(&u, &v)| u + v)
            .collect();
        Self { dims: dims, data: data, ctx: Function::empty(), requires_grad: false }
    }
}

