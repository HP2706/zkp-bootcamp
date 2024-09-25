use std::ops::{Add, Mul, Sub, Div};
use ark_ff::{
    PrimeField,
    fields::{Fp64, MontBackend, MontConfig},
};



#[derive(MontConfig)]
#[modulus = "79"]
#[generator = "3"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;

pub const FR_modulus: i32 = 79;

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub fn lagrange_interpolate(
    f: Vec<Point<Fr>>
) -> Vec<Fr> 
{
    let n = f.len();
    let mut coefficients = vec![Fr::from(0); n];

    for i in 0..n {
        let mut term = vec![Fr::from(0); n];
        term[0] = Fr::from(1);
        let mut denominator = Fr::from(1);

        for j in 0..n {
            if i != j {
                denominator *= f[i].x - f[j].x;
                let mut new_term = vec![Fr::from(0); n];
                for k in (1..n).rev() {
                    new_term[k] = term[k-1] - f[j].x * term[k];
                }
                new_term[0] = -f[j].x * term[0];
                term = new_term;
            }
        }

        for k in 0..n {
            coefficients[k] += f[i].y * term[k] / denominator;
        }
    }

    coefficients
}