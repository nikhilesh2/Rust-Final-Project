// fn main() {
//     println!("Hello, world!");
// }

use std::thread;
use std::time::Duration;





fn main() {
    const NROW:usize = 10;
    const NCOL:usize = 10;

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

    for i in 0..NROW {
        for j in 0..NCOL {
            for k in 0..NROW {
                output_array_c[i][j]+=input_array_a[i][k]*input_array_b[k][j]*2.0;
            }
        }
    }

    let mut totalSum = 0.0;
    // Test Results
    for i in 0..NROW {
        for j in 0..NCOL {
            totalSum+=output_array_c[i][j];
        }
    }
    println!("{}", totalSum);


    
    // let ys: [i32; 500] = [0; 500];
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
}