// fn main() {
//     println!("Hello, world!");
// }

use std::thread;
use rayon::prelude::*;
extern crate scoped_threadpool;
use scoped_threadpool::Pool;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};





fn main() {
    const NROW:usize = 128;
    const NCOL:usize = 128;
    const NUM_THREADS:usize = 8;

    let mut input_array_a = vec![vec![0.0f64; NCOL]; NROW];
    let mut input_array_b = vec![vec![0.0f64; NCOL]; NROW];
    let numbers = Arc::new(Mutex::new(vec![1, 2, 3]));

    

    // Arc(vec![vec![0.0f64; NCOL]; NROW]);
    let mut output_array_c = vec![vec![0.0f64; NCOL]; NROW];

    let mut vec = vec![0; NROW];
    let mut sum:f64 = 2.0;
    // initialize array
    for i in 0..NROW {
        for j in 0..NCOL {
            input_array_a[i][j]= (i*NCOL+j) as f64;
            input_array_b[i][j]= (j*NCOL+j) as f64;
            output_array_c[i][j]= 0.0;
        }
        vec[i] = i;
    }


    let numbers = Arc::new(Mutex::new(vec![1, 2, 3]));

    let mut threads = vec![];
    for i in 0..3 {
        let number = numbers.clone();

        let cur = thread::spawn(move|| {
            let mut array = number.lock().unwrap();

            array[i] += 1;

            println!("numbers[{}] is {}", i, array[i]);
        });
        threads.push(cur);
    }

    for i in threads {
        let _ = i.join();
    }

    // let mut input_array_a_test = Arc::new(Mutex::new(vec![vec![0.0f64; NCOL]; NROW]));
    // let mut input_array_a_test = Arc::new(Mutex::new(input_array_a));
    // let mut input_array_b_test = Arc::new(Mutex::new(input_array_b));
    // let mut input_array_c_test = Arc::new(Mutex::new(output_array_c));

    
    // let mut threads = vec![];
let mut total_sum = 0.0;
let start = Instant::now();
let mut idx:i16 = 0;
output_array_c.par_chunks_mut(16).for_each(|chunk| 
        for i in 0..chunk.len() {
            // println!("chunk length = {}", chunk.len());
            for j in 0..NCOL {
                for k in 0..NROW {
                    chunk[i][j]+=input_array_a[i][k]*input_array_b[k][j]*2.0;
                }
            }
        }
    );
let duration = start.elapsed();

    // for i in 0..NROW {
    //     for j in 0..NCOL {
    //         println!("{:?}", output_array_c[i][j]);   
    //     }
    // }


    // for x in 0..NUM_THREADS {
    //     println!("x={}", x);
    //     let my_a = input_array_a_test.clone();
    //     let my_b = input_array_b_test.clone();
    //     let my_c = input_array_c_test.clone();

    //     let cur = thread::spawn(move|| {
    //         let mut array_a = my_a.lock().unwrap();
    //         let mut array_b = my_b.lock().unwrap();
    //         let mut array_c = my_c.lock().unwrap();
    //         let mut partial_sum = 0.0;

    //         let chunk_size = NROW / NUM_THREADS;
    //         // println!("CHUNK SIZE {}", chunk_size);
    //         let start_index = x * chunk_size;
    //         let mut last_index = (x + 1) * chunk_size;
    //         if(x == NUM_THREADS - 1) {
    //             last_index = NROW;
    //         }
    //         // println!("Last index {}", last_index);
    //         for i in start_index..last_index {
    //             for j in 0..NCOL {
    //                 for k in 0..NROW {
    //                     array_c[i][j]+=array_a[i][k]*array_b[k][j]*2.0;
    //                 }
    //                 partial_sum+=array_c[i][j];
    //             }
    //         }
           
            
    //         // println!("numbers[{}] is {}", x, array[x]);
    //         println!("sum = {}", total_sum);
    //         partial_sum
    //     });
    //     threads.push(cur);
    // }

    // // println!("sum = {}", array_c[0][0]);
    // for i in threads {
    //     let partial_sum = i.join().unwrap();
    //     println!("Parital sum = {}", partial_sum);
    //     total_sum += partial_sum;
    // }
    // let duration = start.elapsed();
    // println!("total_sum = {}", total_sum);
    // println!("Debug: {:?}", duration); 
   
   
   
   
   
   
    // println!("Time taken: {} ms",
    //          (duration.as_secs() * 1_000) + (duration.subsec_nanos() / 1_000_000) as u64);




    // Paralleize
    // for i in 0..NROW {
    //     for j in 0..NCOL {
    //         for k in 0..NROW {
    //             println!("{}", input_array_a[i][k]*input_array_b[k][j]*2.0);
    //             // output_array_c[i][j]+=input_array_a[i][k]*input_array_b[k][j]*2.0;
    //         }
    //     }
    // }
    


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