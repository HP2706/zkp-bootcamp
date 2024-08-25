use crate::conversion::*;
use solana_program::alt_bn128::prelude::{
    alt_bn128_addition, alt_bn128_multiplication,
};
use ark_bn254::{G1Affine, G2Affine, Fr, Fq, G1Projective};
use ark_ff::BigInt;
use ark_ec::{AffineRepr, CurveGroup};
use anyhow::Result;
use crate::conversion::bytes_to_g1;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize,Compress,Validate};
use std::ops::Neg;

pub struct EcMath {}


impl EcMath {

    pub fn add(a: &G1Affine, b: &G1Affine) -> Result<[u8; 64]> {
        let a_bytes = g1_to_bytes(&a);
        let b_bytes = g1_to_bytes(&b);

        // Prepare input for alt_bn128_addition
        let input: Vec<u8> = a_bytes.iter().chain(b_bytes.iter()).cloned().collect();
        let bytes_g1_point = alt_bn128_addition(&input)?;
        Ok(bytes_g1_point.try_into().unwrap())
    }


    pub fn negate_g1(a : &[u8; 64]) -> Result<[u8; 64]> {
        let proof_a = bytes_to_g1(a)?;
        let negated_proof_a = proof_a.neg();
        let negated_proof_a_bytes = g1_to_bytes(&negated_proof_a);
        Ok(negated_proof_a_bytes)
    }


    pub fn mul_g1(a: &G1Affine, scaler : BigInt<4>) -> Result<[u8; 64]> {
        let a_bytes = g1_to_bytes(&a);
        let scaler_bytes = bigint_to_bytes(scaler);

        // Prepare input for alt_bn128_multiplication
        let input: Vec<u8> = a_bytes.iter().chain(scaler_bytes.iter()).cloned().collect();
        let bytes_g1_point = alt_bn128_multiplication(&input)?;
        Ok(bytes_g1_point.try_into().unwrap())
    }

    pub fn mul_g2(a: &G2Affine, scaler : BigInt<4>) -> Result<[u8; 128]> {
        // we need to use arkworks here;

        let a = a.mul_bigint(scaler).into_affine();
        Ok(g2_to_bytes(&a))
    }

}