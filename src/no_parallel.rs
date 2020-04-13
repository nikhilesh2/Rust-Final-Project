use std::time::{Duration, Instant};
fn main() {
    const NROW:usize = 256;
    const NCOL:usize = 256;
    const NUM_THREADS:usize = 8;

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

    let start = Instant::now();
    for i in 0..NROW {
        for j in 0..NCOL {
            for k in 0..NROW {
                output_array_c[i][j]+=input_array_a[i][k]*input_array_b[k][j]*2.0;
            }
        }
    }
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