/*  Copyright [2022] [Wilhelm Ågren]
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 *
 * The following code snippet loops over a 2D matrix in two different ways and calculates 
 * the time it takes to access all elements and sum them. The two variants are:
 *  > column-wise, and
 *  > row-wise.
 *
 *  Supposedly, according to other sources, row-wise should be the optimal choice
 *  because of how the Vec struct is layed out in contiguous memory. We achieve
 *  more CPU cache hits when iterating over rows first instead of columns.
 *
 *  For example, when MATRIX_DIM = 1e4 we iterate and access the Vec 2 seconds
 *  faster in row-wise format compared to column.
 *
 *  Reference: https://pytorch.org/blog/tensor-memory-format-matters/
 *
 */
use std::time::Instant;
use rand::Rng;

static MATRIX_DIM: usize = 8000;

fn loop_row(v: &mut Vec<Vec<f32>>) {
    let mut sum: f32 = 0.0;
    for i in 0..MATRIX_DIM {
        for j in 0..MATRIX_DIM {
            sum += v[i][j];
        }
    }
}

fn loop_col(v: &mut Vec<Vec<f32>>) {
    let mut sum: f32 = 0.0;
    for j in 0..MATRIX_DIM {
        for i in 0..MATRIX_DIM {
            sum += v[i][j];
        }
    }
}

fn time_func(func: fn(&mut Vec<Vec<f32>>), v: &mut Vec<Vec<f32>>, func_name: &str) {
    println!("Func name argument: {}", func_name);
    let mut times: f64 = 0.0; 
    for i in 0..10 {
        let now = Instant::now();
        func(v);
        let t: f64 = now.elapsed().as_millis() as f64;
        println!("{} iteration: {} took {} ms", func_name, i, t);
        times += t;
    }
    println!("{} average loop time {} ms", func_name, times / 10.0);
}

fn main() {
    println!("Vector loop layout performance testing...");

    let mut rng = rand::thread_rng();
    let mut v: Vec<Vec<f32>> = vec![vec![0.0; MATRIX_DIM]; MATRIX_DIM];
    for _ in 0..160000 {
        let x: usize = rng.gen_range(0..MATRIX_DIM);
        let y: usize = rng.gen_range(0..MATRIX_DIM);
        v[x][y] = rng.gen_range(0.0..10000.0);
    }

    time_func(loop_row, &mut v, "loop_row");
    time_func(loop_col, &mut v, "loop_col");
}
