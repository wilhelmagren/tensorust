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
//
//  The following code compares two programmatic ideas of looping over a nested Vec
//  (a 2D matrix): either rows first or columns first. Supposedly one of the other 
//  is faster. According to D. Matani and S. Subramanian the Tensor's memory format
//  can significantly impact the inference and execution speed of neural networks 
//  when using the CPU. "For maximum efficiency, one should always access data in 
//  the same format in which it is stored." [1]. Matani and Subramanian reach the
//  conclusion that a row-wise inner loop is significantly faster, as it allows
//  for more CPU cache hits. This is for a generic Vec in C++, but the same seems
//  to hold for the Vec struct in Rust.
//
//  The general trend seems to be that as the dimensions of the matrix grows, so 
//  does the difference in looping time for the two implementations. 
//  When performing this comparison on a matrix with dimensions 5000x5000 we can
//  already see that the cache hits give us an approximate 20% improvement for the
//  row-wise implementation. This is simply too much to call insignificant, even
//  though I haven't performed the statistical tests needed to state significance.
//  But just imagine when we have 4D Tensor's with total amount of elements in the
//  same ball-park; if we can spend 20% less time during inference and training,
//  why shouldn't we?
//
//  References
//  ----------
//  [1] Dhruv Matani, Suraj Subramanian. PyTorch Foundation. 
//  https://pytorch.org/blog/tensor-memory-format-matters/, accessed: 21-09-22.
//

use rand::Rng;
use std::time::Instant;
use chrono::offset::Local;

macro_rules! info {
    ($($arg:tt)*) => {
        println!(
            "{}  [INFO]  {}", 
            Local::now().format("%Y-%m-%d  %H:%M:%S"), 
            format!($($arg)*)
        );
    }
}

fn loop_row(
    vec: &mut Vec<Vec<f32>>,
    matrix_dim: usize
) -> f32 {
    let mut sum: f32 = 0.0;
    for i in 0..matrix_dim {
        for j in 0..matrix_dim {
            sum += vec[i][j];
        }
    }
    sum
}

fn loop_col(
    vec: &mut Vec<Vec<f32>>,
    matrix_dim: usize
) -> f32 {
    let mut sum: f32 = 0.0;
    for i in 0..matrix_dim {
        for j in 0..matrix_dim {
            sum += vec[j][i];
        }
    }
    sum
}

fn time_func(
    func: fn(&mut Vec<Vec<f32>>, usize) -> f32, 
    vec: &mut Vec<Vec<f32>>,
    func_name: &str,
    matrix_dim: usize
) -> f64 {
    info!("Started working on {}...", func_name);
    let mut times: f64 = 0.0; 
    for i in 0..10 {
        let now = Instant::now();
        let _sum: f32 = func(vec, matrix_dim);
        let t: f64 = now.elapsed().as_millis() as f64;
        info!("iteration: {} took {} ms", i + 1, t);
        times += t;
    }
    let avg_time_ms: f64 = times / 10.0;
    info!("{} done! Average loop time {} ms", func_name, times / 10.0);
    avg_time_ms
}

fn compare_order_performance(
    matrix_dim: usize
) {
    info!("Running the row-vs-column-looping experiment...");
    info!("Randomly generating a {}x{} matrix", matrix_dim, matrix_dim);
    let mut rng = rand::thread_rng();
    let mut vec: Vec<Vec<f32>> = vec![vec![0.0; matrix_dim]; matrix_dim];
    for _ in 0..1000000 {
        let x: usize = rng.gen_range(0..matrix_dim);
        let y: usize = rng.gen_range(0..matrix_dim);
        vec[x][y] = rng.gen_range(0.0..10000.0);
    }
    let row_ms: f64 = time_func(loop_row, &mut vec, "loop_row", matrix_dim);
    let col_ms: f64 = time_func(loop_col, &mut vec, "loop_col", matrix_dim);
    info!("Done with all iterations!");
    if row_ms < col_ms {
        info!("Row-wise is on average {:.3} times faster.", col_ms / row_ms);
    } else {
        info!("Column-wise is on average {:.3} times faster.", row_ms / col_ms);
    }
}

fn main() {
    for md in vec![100, 500, 1000, 2500, 5000, 10000, 15000] {
        compare_order_performance(md);
    }
}
