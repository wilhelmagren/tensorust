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
//  Last updated: 22-10-08
//

use crate::Tensor;

pub struct Function {
    parents: Vec<Tensor>,
    saved_tensors: Vec<Tensor>,
    requires_grad: bool
}

impl Function {
    pub fn add(u: Tensor, v: Tensor) -> Self {
        Self { parents: vec![u, v], saved_tensors: vec![], requires_grad: false }
    }

    pub fn empty() -> Self {
        Self { parents: vec![], saved_tensors: vec![], requires_grad: false  }
    }
}

