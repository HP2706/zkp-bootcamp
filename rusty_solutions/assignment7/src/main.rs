/* # EXERCISE 2
def interpolate_column(col, upper_bound : int) -> np.array:
    xs = np.arange(1, upper_bound + 1)
    lagrange_poly = lagrange_interpolate(list(zip(xs, col)))
    return lagrange_poly
    
    #return galois.lagrange_poly(xs, col)
import numpy as np
import random
import functools
import typing as tp

def matrix_to_polynomials(matrix : np.array) -> np.array:
    func = functools.partial(interpolate_column, upper_bound=matrix.shape[1])
    return np.apply_along_axis(func, 0, matrix)
    
# Define the matrices
A = np.array([[0,0,3,0,0,0],
               [0,0,0,0,1,0],
               [0,0,1,0,0,0]])

B = np.array([[0,0,1,0,0,0],
               [0,0,0,1,0,0],
               [0,0,0,5,0,0]])

C = np.array([[0,0,0,0,1,0],
               [0,0,0,0,0,1],
               [-3,1,1,2,0,-1]])

# pick values for x and y
x = 100
y = 100

# this is our orignal formula
out = 3 * x * x * y + 5 * x * y - x- 2*y + 3
# the witness vector with the intermediate variables inside
v1 = 3*x*x
v2 = v1 * y
w = np.array([1, out, x, y, v1, v2])

result = C.dot(w) == np.multiply(A.dot(w),B.dot(w))
assert result.all(), "result contains an inequality"

def qap(
    A : np.array, 
    B : np.array, 
    C : np.array, 
    w : np.array, 
    point : tuple[int, int]
):

    A_poly = tp.cast(np.array, matrix_to_polynomials(A))
    B_poly = tp.cast(np.array, matrix_to_polynomials(B))
    C_poly = tp.cast(np.array, matrix_to_polynomials(C))

    n_rows = A.shape[0]
    term1 = C_poly.dot(w)
    term2 = A_poly.dot(w)
    term3 = B_poly.dot(w)
    
    x = sp.Symbol('x')
    t = sp.prod(x - i for i in range(1, n_rows + 1))
    
    h = (term1 * term2 - term3) // t
    
    left_side = term1 * term2
    right_side = term3 + h * t
    
    left_side_fn = sp.lambdify(x, left_side)
    right_side_fn = sp.lambdify(x, right_side)
    
    assert np.isclose(left_side_fn(point[0]), right_side_fn(point[0]), rtol=1e-9), "Left side is not approximately equal to right side"
    return True

print(qap(A, B, C, w, (5, 100))) */
pub mod utils;
use crate::utils::{lagrange_interpolate, Point, Fr, FR_modulus};
use ark_ff::PrimeField;
use ndarray::{
    Array2, 
    array, 
    Array1, 
    Array,
    Dimension,
};


fn matrix_to_polynomials(mat: Array2<Fr>) -> Array2<Fr> {
    let n_rows = mat.shape()[0];
    let n_cols = mat.shape()[1];
    let mut poly = Vec::new();
    for col in 0..n_cols {
        let x = Array1::from_vec((1..=n_rows).map(|i| i as u64).collect());
        let y = mat.column(col).to_vec();

        let points = x.iter().zip(y.iter()).map(|(x, y)| Point { x: Fr::from(*x), y: *y }).collect();
        let lagrange_poly = lagrange_interpolate(points);
        poly.extend(lagrange_poly);
    }
    Array2::from_shape_vec((n_cols, n_rows), poly).unwrap()
}

fn main() {

    let A: Array2<i32> = array![
        [0, 0, 3, 0, 0, 0],
        [0, 0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0, 0],
    ];

    let B: Array2<i32> = array![
        [0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0],
        [0, 0, 0, 5, 0, 0],
    ];

    // Shape: 3x6
    let C: Array2<i32> = array![
        [0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 1],
        [-3, 1, 1, 2, 0, -1],
    ];



    let x = 100;
    let y = 100;

    // this is our orignal formula
    let out = 3 * x * x * y + 5 * x * y - x- 2*y + 3;
    // the witness vector with the intermediate variables inside
    let v1 = 3*x*x;
    let v2 = v1 * y;
    let w : Array1<i32> = array![1, out, x, y, v1, v2];

    //result = C.dot(w) == np.multiply(A.dot(w),B.dot(w))
    //assert result.all(), "result contains an inequality"
    assert!(C.dot(&w).eq(&(A.dot(&w) * B.dot(&w))));

    fn To_Field<D: Dimension>(mat: Array<i32, D>) -> Array<Fr, D> {
        //(mat + curve_order) % curve_order
        mat.map(|x| Fr::from((x + FR_modulus as i32) % FR_modulus as i32))
    }

    let n_rows = A.shape()[0];
    let n_cols = A.shape()[1];
    let F_A = To_Field(A);
    let F_B = To_Field(B);
    let F_C = To_Field(C);
    let F_w = To_Field(w).into_shape_with_order((n_cols, 1)).unwrap();

    let F_A_poly = matrix_to_polynomials(F_A);
    let F_B_poly = matrix_to_polynomials(F_B);
    let F_C_poly = matrix_to_polynomials(F_C);

    let term1 = F_C_poly.t().dot(&F_w);
    let term2 = F_A_poly.t().dot(&F_w);
    let term3 = F_B_poly.t().dot(&F_w);

    let t: Array1<Fr> = Array1::from_vec((1..=n_rows).map(|i| Fr::from(i as u64)).collect());
    
    // Fix: Ensure all terms are 1-dimensional and perform element-wise operations
    let term1 = term1.to_shape(n_rows).unwrap();
    let term2 = term2.to_shape(n_rows).unwrap();
    let term3 = term3.to_shape(n_rows).unwrap();

    // Calculate h using element-wise operations
    let h = (&term1 * &term2 - &term3) / &t;

    println!("shape term1: {:?}", term1.shape());
    println!("shape term2: {:?}", term2.shape());
    println!("shape term3: {:?}", term3.shape());
    println!("shape t: {:?}", t.shape());
    println!("shape h: {:?}", h.shape());

    let left_side = &term1 * &term2;
    let right_side = &term3 + &h * &t;

    println!("left_side: {:?}", left_side);
    println!("right_side: {:?}", right_side);

}
