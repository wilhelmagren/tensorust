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
//  Last updated: 22-10-10
//

use std::fmt;
use crate::Tensor;

struct FunctionTypeError;

impl fmt::Display for FunctionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No valid FunctionType found.")
    }
}

impl fmt::Debug for FunctionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

#[derive(Debug)]
pub enum FunctionType {
    Add,
    Sub,
    Mul,
    Dot,
    Conv2d,
    Pool2d,
    Null
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Function<'a> {
    operation: FunctionType,
    parents: Vec<&'a Tensor<'a>>,
    saved_tensors: Vec<&'a Tensor<'a>>,
    requires_grad: bool
}

fn requires_grad<'a>(tensors: &'a Vec<&'a Tensor<'a>>) -> bool {
    for &t in tensors.iter() { if !t.requires_grad() { return false; } }
    return true;
}

fn add<'a>(ctx: &'a Function) -> Tensor<'a> {
    let u: &'a Tensor = ctx.parents[0];
    let v: &'a Tensor = ctx.parents[1];
    assert_eq!(*u.dims(), *v.dims());

    let dims: Vec<usize> = u.dims().clone();
    let data: Vec<f32> = u.data().iter()
        .zip(v.data().iter())
        .map(|(&a, &b)| a + b)
        .collect();
    Tensor { dims: dims, data: data, ctx: ctx,
    requires_grad: requires_grad(&ctx.parents)
    }
}

impl<'a> Function<'a> {
    pub fn null() -> &'a Self {
        &Self { operation: FunctionType::Null, parents: vec![],
        saved_tensors: vec![], requires_grad: false }
    }
    
    pub fn add(u: &'a Tensor, v: &'a Tensor) -> Self {
        Self { operation: FunctionType::Add, parents: vec![u, v],
        saved_tensors: vec![], requires_grad: false }
    }

    pub fn forward(&self) -> Result<Tensor, FunctionTypeError> {
        match self.operation {
            FunctionType::Add => Ok(add(self)),
            _ => Err(FunctionTypeError),
        }
    }
}


