use solana_program::alt_bn128::{self, prelude::*};
use solana_program::alt_bn128::prelude::{
    alt_bn128_addition, alt_bn128_multiplication, alt_bn128_pairing,
};
use ark_bn254::{G1Affine, G2Affine, Fr, Fq, G1Projective};
use ark_ff::{BigInteger, BigInteger256, Field, PrimeField, BigInt, Fp256};
use ark_ec::{AffineRepr, CurveGroup, models::CurveConfig};

use assignment5::conversion::{g1_to_bytes, scalar_to_bytes, g2_to_bytes, bigint_to_bytes, bytes_to_g1};
use assignment5::verifier::{Verifier, VerificationKeys};
use assignment5::ecmath::EcMath;
use std::ops::Sub;

fn main() {
    test_verifier();

}

fn test_verifier(){
    // Generate two G1 points
    let g1_generator = G1Affine::generator();
    let g2_generator = G2Affine::generator();
    let scalar_alpha1 = 1;
    let scalar_beta2 = 1;
    let scalar_gamma2 = 1;
    let scalar_delta2 = 1;

    let scalar_x1 = Fq::from(1);
    let scalar_x2 = Fq::from(1);
    let scalar_x3 = Fq::from(-3);

    println!("Scalar x3: {:?}", scalar_x3);

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
    
    /* uint256 scalar_a1 = 2;
        uint256 scalar_b2 = 1;
        uint256 scalar_c1 = 2;

        uint256 scalar_x1 = 1;
        uint256 scalar_x2 = 1;
        uint256 scalar_x3 = Pairing.CurveOrder-3; //use additive inverse of 3

        Pairing.G1Point memory A1 = Pairing.create_G1Point(scalar_a1);
        Pairing.G2Point memory B2 = Pairing.get_g2_1();// assumes we have scalar_b2 =1
        Pairing.G1Point memory C1 = Pairing.create_G1Point(scalar_c1);
 */

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




fn test(){
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

    let scalar_bytes = bigint_to_bytes(BigInt::<4>::from(11 as u64));
    println!("Scalar bytes: {:?}", scalar_bytes);

    let input2 : Vec<u8> = vk_alpha_g1_bytes.iter().chain(scalar_bytes.iter()).cloned().collect();
    match alt_bn128_multiplication(&input2) {
        Ok(result) => println!("Multiplication result: {:?}", result),
        Err(e) => println!("Multiplication error: {:?}", e),
    }

}