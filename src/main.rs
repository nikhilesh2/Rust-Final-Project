// use std::thread;g
// extern crate scoped_threadpool;
// use scoped_threadpool::Pool;
// use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use std::time::{Instant};

fn main() {
    const NROW:usize = 256;
    const NCOL:usize = 256;

    let mut input_array_a = vec![vec![0i32; NCOL]; NROW];
    let mut input_array_b = vec![vec![0i32; NCOL]; NROW];
    let mut output_array_c = vec![vec![0i32; NCOL]; NROW];

    // initialize array
    for i in 0..NROW {
        for j in 0..NCOL {
            input_array_a[i][j]= (i*NCOL+j) as i32;
            input_array_b[i][j]= (j*NCOL+j) as i32;
            output_array_c[i][j]= 0;
        }
    }

    // Parallelization
    let start = Instant::now();
    output_array_c.par_chunks_mut(NROW).for_each(|chunk| 
            for i in 0..chunk.len() {
                for j in 0..NCOL {
                    for k in 0..NROW {
                        chunk[i][j]+=input_array_a[i][k]*input_array_b[k][j]*2;
                    }
                }
            }
        );
    let duration = start.elapsed();

    let mut total_sum = 0.0;
    // Test Results
    for i in 0..NROW {
        for j in 0..NCOL {
            total_sum+=output_array_c[i][j] as f64;
        }
    }
    println!("{}",  format!("{:+e}", total_sum));
    println!("Time taken: {:?}", duration); 

    rect();

}

fn rect() {
    let NSTEPS = 8388600 as i64;
    let NITER = 8388600 as i64;
    let p_start = 0 as i32;
    let p_end = 10 as i32;

    let h = ((p_end as f64 - p_start as f64)/NSTEPS as f64) as f64;

    // start timer
    let start = Instant::now();

    /* Parallel Implementation
     * 
     * Start by taking a parallel iterator across the total iterations. Rust's Rayon crate handles parallel iterators quite nicely. (Good call on that, Nik.)
     * We can then, super easily, map our calculation across each iteration, and then sum that up. And since we're working on a parallel iterator, both the
     * map and the sum are done in parallel.
    */
    let area :f64 = (0..NITER).into_par_iter().map(|x| {
        (x as f64 * h).cos() * h // in Rust, statements that do not end with a semicolon are return statements
    }).sum();

/*
    // serial implementation
    let mut area = 0.0 as f64;
    let mut p_current = p_start as f64;
    let mut f_result = 0.0 as f64;

    for i in 0..NITER {
        p_current = i as f64 * h;
        f_result = p_current.cos();
        area += f_result*h;
    }
*/

    // end timer
    let duration = start.elapsed();
    println!("Rect (or Reimann) Sum of Cosine from 0 to 10. Resulting area : {:.2}", area);
    println!("Time taken: {:?}", duration); 
}