/* 
from py_ecc.bn128 import (
    G1, 
    G2, 
    multiply, 
    add, 
    pairing, 
    bn128_curve,
    eq,
    neg
)
from dataclasses import dataclass
import numpy as np


def verifier(
    A1 : int, 
    B2 : int, 
    C1 : int, 
    x1 : int, 
    x2 : int, 
    x3 : int
) -> bool:    
    '''
    Checks that:
    0 = pairing(B2, neg(A1)) + pairing(beta_2, alpha_1) + pairing(gamma_2, X1) + pairing(delta_2, C1)
    '''
    _alpha_1 = 1
    _beta_2 = 1
    _gamma_2 = 1
    _delta_2 = 1
    
    alpha_1 = multiply(G1, _alpha_1)
    beta_2 = multiply(G2, _beta_2)
    gamma_2 = multiply(G2, _gamma_2)
    delta_2 = multiply(G2, _delta_2)
    
    _X1 = (x1 + x2 + x3) % bn128_curve.curve_order
    
   
    X1 = multiply(G1, _X1)
    return eq(
        pairing(B2, A1), 
        pairing(beta_2, alpha_1) * pairing(gamma_2, X1) * pairing(delta_2, C1)
    )
*/
use ark_ec::{AffineRepr, CurveGroup, models::CurveConfig};
use ark_ff::{BigInteger, BigInteger256, Field, PrimeField, BigInt, Fp256};
use ark_bn254::{G1Affine, G2Affine, Bn254, Fr};
use ark_bn254::Fq2Config;
use solana_program::alt_bn128::prelude::{alt_bn128_addition, alt_bn128_multiplication, alt_bn128_pairing};
use crate::conversion::{g1_to_bytes, scalar_to_bytes, g2_to_bytes, bigint_to_bytes};
use anyhow::Result;
use std::ops::Neg;
use ark_serialize::CanonicalDeserialize;


pub struct VerificationKeys {
    pub alpha_1: i64,
    pub beta_2: i64,
    pub gamma_2: i64,
    pub delta_2: i64,
}

pub struct Verifier {
    pub scalar_keys : VerificationKeys,
    pub alpha_g1 : G1Affine,
    pub beta_g2 : G2Affine,
    pub gamma_g2 : G2Affine,
    pub delta_g2 : G2Affine,
}

impl Verifier {


    pub fn create_g1_point(g1_generator : G1Affine, scalar : i64) -> G1Affine {
        g1_generator.mul_bigint(BigInt::<4>::from(scalar as u64)).into_affine()
    }

    pub fn create_g2_point(g2_generator : G2Affine, scalar : i64) -> G2Affine {
        g2_generator.mul_bigint(BigInt::<4>::from(scalar as u64)).into_affine()
    }

    pub fn new(keys : VerificationKeys) -> Self {
        
        let g1_generator = G1Affine::generator();
        let g2_generator = G2Affine::generator();

        let alpha_g1 = Self::create_g1_point(g1_generator, keys.alpha_1);
        let beta_g2 = Self::create_g2_point(g2_generator, keys.beta_2);
        let gamma_g2 = Self::create_g2_point(g2_generator, keys.gamma_2);
        let delta_g2 = Self::create_g2_point(g2_generator, keys.delta_2);
        
        Self {
            scalar_keys : keys,
            alpha_g1 : alpha_g1,
            beta_g2 : beta_g2,
            gamma_g2 : gamma_g2,
            delta_g2 : delta_g2,
        }
    }


    pub fn verify(
        &self, 
        A_G1 : G1Affine, 
        B_G2 : G2Affine, 
        C_G1 : G1Affine, 
        x1 : i32, 
        x2 : i32, 
        x3 : i32
    ) -> Result<bool> {
        use ark_bn254::Fr;

        let _X1 = Fr::from(x1 as u64) + Fr::from(x2 as u64) + Fr::from(x3 as u64);

        let scalar_bytes = bigint_to_bytes(_X1.into_bigint());
        let point_bytes = g1_to_bytes(&G1Affine::generator());
        let input: Vec<u8> = point_bytes.iter().chain(scalar_bytes.iter()).cloned().collect();
        let X1_G1_bytes = alt_bn128_multiplication(&input)?;

        let X1_G1 = G1Affine::deserialize_uncompressed(&X1_G1_bytes[..]).map_err(|e| anyhow::anyhow!("Deserialization error: {:?}", e))?;

        /* let pairing_input = [
            self.proof_a.as_slice(),
            self.proof_b.as_slice(),
            self.prepared_public_inputs.as_slice(),
            self.verifyingkey.vk_gamme_g2.as_slice(),
            self.proof_c.as_slice(),
            self.verifyingkey.vk_delta_g2.as_slice(),
            self.verifyingkey.vk_alpha_g1.as_slice(),
            self.verifyingkey.vk_beta_g2.as_slice(),
        ]
        .concat();

        let pairing_res = alt_bn128_pairing(pairing_input.as_slice())
            .map_err(|_| Groth16Error::ProofVerificationFailed)?;
         */

        let neg_A1 = A_G1.neg();

        let pairing_input = [
            g1_to_bytes(&neg_A1).as_slice(),
            g2_to_bytes(&B_G2).as_slice(),
            g1_to_bytes(&C_G1).as_slice(),
            g2_to_bytes(&self.gamma_g2).as_slice(),
            g1_to_bytes(&X1_G1).as_slice(),
            g2_to_bytes(&self.delta_g2).as_slice(),
            g1_to_bytes(&self.alpha_g1).as_slice(),
            g2_to_bytes(&self.beta_g2).as_slice(),
        ]
        .concat();

        let pairing_res = alt_bn128_pairing(pairing_input.as_slice())
            .map_err(|_| anyhow::anyhow!("Pairing failed"))?;
        


        /* X1 = multiply(G1, _X1)
        return eq(
            pairing(B2, A1), 
            pairing(beta_2, alpha_1) * pairing(gamma_2, X1) * pairing(delta_2, C1)
        ) */


        Ok(true)
    }
}



/* function pairing(
        G1Point memory a1,
        G2Point memory a2,
        G1Point memory b1,
        G2Point memory b2,
        G1Point memory c1,
        G2Point memory c2,
        G1Point memory d1,
        G2Point memory d2
    ) internal view returns (bool) {
        G1Point[4] memory p1 = [a1, b1, c1, d1];
        G2Point[4] memory p2 = [a2, b2, c2, d2];

        uint256 inputSize = 24;
        uint256[] memory input = new uint256[](inputSize);

        for (uint256 i = 0; i < 4; i++) {
            uint256 j = i * 6;
            input[j + 0] = p1[i].X;
            input[j + 1] = p1[i].Y;
            input[j + 2] = p2[i].X[1];
            input[j + 3] = p2[i].X[0];
            input[j + 4] = p2[i].Y[1];
            input[j + 5] = p2[i].Y[0];
        }

        uint256[1] memory out;
        bool success; */