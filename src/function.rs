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
//  Last updated: 22-10-09
//

use crate::Tensor;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Function<'a> {
    parents: Vec<&'a Tensor<'a>>,
    saved_tensors: Vec<&'a Tensor<'a>>,
    requires_grad: bool
}

impl<'a> Function<'a> {
    pub fn none() -> Self {
        Self { parents: vec![], saved_tensors: vec![], requires_grad: false }
    }
    
    pub fn add(u: &'a Tensor, v: &'a Tensor) -> Self {
        Self { parents: vec![u, v], saved_tensors: vec![], requires_grad: false }
    }
}

