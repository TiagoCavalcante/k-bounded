use ndarray::{Array, Array1, Array2};
use num_bigint::BigInt;
use num_traits::{One, Zero};

// Define the sigma function that determines matrix entries
fn sigma(i: usize, j: usize, k: isize) -> BigInt {
    if (i as isize - j as isize).abs() <= k {
        BigInt::one()
    } else {
        BigInt::zero()
    }
}

// Create the matrix C based on the sigma function
fn create_matrix_c(n: usize, k: isize) -> Array2<BigInt> {
    Array::from_shape_fn((n, n), |(i, j)| sigma(i, j, k))
}

// Manual matrix multiplication tailored for BigInt types
fn matrix_mult(a: &Array2<BigInt>, b: &Array2<BigInt>) -> Array2<BigInt> {
    let n = a.nrows();
    let mut result = Array2::<BigInt>::zeros((n, n));

    for i in 0..n {
        for j in 0..n {
            let mut sum = BigInt::zero();
            for k in 0..n {
                let product = &a[(i, k)] * &b[(k, j)];
                sum += product;
            }
            result[(i, j)] = sum;
        }
    }
    result
}

// Function to compute the power of a matrix
fn matrix_power(mut a: Array2<BigInt>, mut exp: usize) -> Array2<BigInt> {
    let n = a.nrows();
    let mut result = Array::from_elem((n, n), BigInt::one()); // Identity matrix initialization

    while exp > 0 {
        if exp % 2 == 1 {
            result = matrix_mult(&result, &a);
        }
        a = matrix_mult(&a, &a);
        exp /= 2;
    }
    result
}

// Compute the final result by multiplying the matrix power with a vector of ones and summing the results
fn compute_final_result(n: usize, k: isize) -> BigInt {
    let c = create_matrix_c(n, k);
    let c_power = matrix_power(c, n - 1);
    let u = Array1::from(vec![BigInt::one(); n]); // Column vector of ones

    // Manual multiplication of matrix and vector
    let mut result_vector = Array1::<BigInt>::zeros(n);
    for i in 0..n {
        let mut sum = BigInt::zero();
        for j in 0..n {
            let product = &c_power[(i, j)] * &u[j];
            sum += product;
        }
        result_vector[i] = sum;
    }

    // Sum all elements of the resulting vector
    result_vector.iter().cloned().sum()
}

fn main() {
    let n = 200; // Matrix size
    let k = 199; // Sigma function threshold
    let final_result = compute_final_result(n, k);
    println!("a({n},{k}) = {}", final_result);
}
