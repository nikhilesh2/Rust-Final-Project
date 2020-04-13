// use std::thread;g
// extern crate scoped_threadpool;
// use scoped_threadpool::Pool;
// use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use std::time::{Instant};

fn main() {
    const NROW:usize = 128;
    const NCOL:usize = 128;

    let mut input_array_a = vec![vec![0.0f64; NCOL]; NROW];
    let mut input_array_b = vec![vec![0.0f64; NCOL]; NROW];
    let mut output_array_c = vec![vec![0.0f64; NCOL]; NROW];

    // initialize array
    for i in 0..NROW {
        for j in 0..NCOL {
            input_array_a[i][j]= (i*NCOL+j) as f64;
            input_array_b[i][j]= (j*NCOL+j) as f64;
            output_array_c[i][j]= 0.0;
        }
    }

    // Parallelization
    let start = Instant::now();
    output_array_c.par_chunks_mut(16).for_each(|chunk| 
            for i in 0..chunk.len() {
                for j in 0..NCOL {
                    for k in 0..NROW {
                        chunk[i][j]+=input_array_a[i][k]*input_array_b[k][j]*2.0;
                    }
                }
            }
        );
    let duration = start.elapsed();

    let mut total_sum = 0.0;
    // Test Results
    for i in 0..NROW {
        for j in 0..NCOL {
            total_sum+=output_array_c[i][j];
        }
    }
    println!("{}", total_sum);
    println!("Time taken: {:?}", duration); 

}

fn rect(mut threadcount: i32) {
    let NSTEPS = 8388600;
    let NITER = 8388600;
    let P_START = 0;
    let P_END = 10;

    let mut h = ((P_END - P_START)/NSTEPS) as f64;
    if threadcount == 0 {
        threadcount = 1;
    }

    /*
     need to define a parallel sum here. Something in Rayon, it looks like. sum() somewhere?
     Hmm. Maybe not here. Build a vector and then use the parallel iterator to sum it.
     https://docs.rs/rayon/1.0.3/rayon/iter/trait.ParallelIterator.html#method.sum
    */

    let mut p_current = P_START;
    let mut area = 0.0 as f64;

}