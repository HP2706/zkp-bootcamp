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
use ark_bn254::{G1Affine, G2Affine, Bn254, Fr, Fq};
use ark_bn254::Fq2Config;
use solana_program::alt_bn128::prelude::{alt_bn128_addition, alt_bn128_multiplication, alt_bn128_pairing};
use crate::conversion::{bigint_to_bytes, bytes_to_g1, bytes_to_g2, g1_to_bytes, g2_to_bytes, scalar_to_bytes};
use anyhow::Result;

use std::ops::Neg;
use ark_serialize::CanonicalDeserialize;
use num_bigint::BigUint;
use std::fs::File;
use std::io::Write;

pub struct VerificationKeys {
    pub alpha_1: BigInt<4>,
    pub beta_2: BigInt<4>,
    pub gamma_2: BigInt<4>,
    pub delta_2: BigInt<4>,
}

pub struct Verifier {
    pub scalar_keys : VerificationKeys,
    pub alpha_g1 : G1Affine,
    pub beta_g2 : G2Affine,
    pub gamma_g2 : G2Affine,
    pub delta_g2 : G2Affine,
}

impl Verifier {

    pub fn new(keys : VerificationKeys) -> Self {
        
        let g1_generator = G1Affine::generator();
        let g2_generator = G2Affine::generator();

        let alpha_g1 =g1_generator.mul_bigint(keys.alpha_1);
        let beta_g2 = g2_generator.mul_bigint(keys.beta_2);
        let gamma_g2 = g2_generator.mul_bigint(keys.gamma_2);
        let delta_g2 = g2_generator.mul_bigint(keys.delta_2);
        
        Self {
            scalar_keys : keys,
            alpha_g1 : alpha_g1.into(),
            beta_g2 : beta_g2.into(),
            gamma_g2 : gamma_g2.into(),
            delta_g2 : delta_g2.into(),
        }
    }


    pub fn verify(
        &self, 
        neg_A1 : [u8; 64], 
        B_G2 : [u8; 128], 
        C_G1 : [u8; 64], 
        x1 : Fr, 
        x2 : Fr, 
        x3 : Fr
    ) -> Result<bool> {
        
        let X1: Fr = x1 + x2 + x3;
        let X1_G1 = G1Affine::generator().mul_bigint(X1.into_bigint()).into_affine();
       
        let pairing_input = [
            neg_A1.as_slice(),
            B_G2.as_slice(),
            g1_to_bytes(&self.alpha_g1).as_slice(),
            g2_to_bytes(&self.beta_g2).as_slice(),
            g1_to_bytes(&X1_G1).as_slice(),
            g2_to_bytes(&self.delta_g2).as_slice(),
            C_G1.as_slice(),
            g2_to_bytes(&self.gamma_g2).as_slice(),
        ]
        .concat();

        let pairing_res = alt_bn128_pairing(pairing_input.as_slice())
            .map_err(|_| anyhow::anyhow!("Pairing failed"))?;
        
        if pairing_res[31] != 1 {
            return Err(anyhow::anyhow!("Proof verification failed"));
        }
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