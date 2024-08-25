use solana_program::alt_bn128::{self, prelude::*};
use solana_program::alt_bn128::prelude::{
    alt_bn128_addition, alt_bn128_multiplication, alt_bn128_pairing,
};
use ark_bn254::{G1Affine, G2Affine, Fr, Fq, G1Projective};
use ark_ff::{BigInteger, BigInteger256, Field, PrimeField, BigInt, Fp256};
use ark_ec::{AffineRepr, CurveGroup, models::CurveConfig};

use assignment5::conversion::{g1_to_bytes, scalar_to_bytes, g2_to_bytes};




fn main() {
    // Generate two G1 points
    let g1_generator = G1Affine::generator();
    let vk_alpha_g1 = g1_generator.mul_bigint(BigInt::<4>::from(1 as u64));
    let another_g1 = g1_generator.mul_bigint(BigInt::<4>::from(10 as u64));

    // Convert G1 points to bytes in the correct format
    let vk_alpha_g1_bytes = g1_to_bytes(&vk_alpha_g1.into_affine());
    let another_g1_bytes = g1_to_bytes(&another_g1.into_affine());

    // Prepare input for alt_bn128_addition
    let input: Vec<u8> = vk_alpha_g1_bytes.iter().chain(another_g1_bytes.iter()).cloned().collect();

    // Perform addition
    match alt_bn128_addition(&input) {
        Ok(result) => println!("Addition result: {:?}", result),
        Err(e) => println!("Addition error: {:?}", e),
    }

    let scalar_bytes = scalar_to_bytes(11 as u64);
    println!("Scalar bytes: {:?}", scalar_bytes);

    let input2 : Vec<u8> = vk_alpha_g1_bytes.iter().chain(scalar_bytes.iter()).cloned().collect();

    match alt_bn128_multiplication(&input2) {
        Ok(result) => println!("Multiplication result: {:?}", result),
        Err(e) => println!("Multiplication error: {:?}", e),
    }



}