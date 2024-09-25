use solana_program::alt_bn128::{self, prelude::*};
use solana_program::alt_bn128::prelude::{
    alt_bn128_addition, alt_bn128_multiplication, alt_bn128_pairing,
};
use ark_bn254::{G1Affine, G2Affine, Fr, Fq, G1Projective};
use ark_ff::BigInt;
use ark_ec::{AffineRepr, CurveGroup, models::CurveConfig};

use crate::conversion::{g1_to_bytes, scalar_to_bytes, g2_to_bytes, bigint_to_bytes, bytes_to_g1};
use crate::verifier::{Verifier, VerificationKeys};
use crate::ecmath::EcMath;
use std::ops::Sub;

fn main() {
    //test_verifier();

    println!("{:?}", test_verifier());
}

fn test_verifier(){
    // Generate two G1 points
    let g1_generator = G1Affine::generator();
    let g2_generator = G2Affine::generator();
    let scalar_alpha1 = 1;
    let scalar_beta2 = 1;
    let scalar_gamma2 = 1;
    let scalar_delta2 = 1;

    let scalar_x1 = Fr::from(1);
    let scalar_x2 = Fr::from(1);
    let scalar_x3 = Fr::from(-3);

    let Verifier = Verifier::new(VerificationKeys {
        alpha_1: BigInt::<4>::from(scalar_alpha1 as u64),
        beta_2: BigInt::<4>::from(scalar_beta2 as u64),
        gamma_2: BigInt::<4>::from(scalar_gamma2 as u64),
        delta_2: BigInt::<4>::from(scalar_delta2 as u64),
    });

    let a1 = EcMath::mul_g1(&g1_generator, BigInt::<4>::from(2 as u64)).unwrap();

    let neg_a1 = EcMath::negate_g1(&a1).unwrap();
    let b2 = EcMath::mul_g2(&g2_generator, BigInt::<4>::from(1 as u64)).unwrap();
    let c1 = EcMath::mul_g1(&g1_generator, BigInt::<4>::from(2 as u64)).unwrap();
    
    let result = Verifier.verify(
        neg_a1, 
        b2,
        c1,
        scalar_x1,
        scalar_x2,
        scalar_x3
    );

    println!("Verification result: {:?}", result);
}
