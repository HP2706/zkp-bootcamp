pub mod keys;



use anchor_lang::prelude::*;
use anchor_lang::solana_program::alt_bn128::{self, prelude::*};
use anchor_lang::solana_program::alt_bn128::{PodG1, PodG2};

declare_id!("Gv1oWEc6xhigsbmsLua1YEBr6Yc53ffCQ1xdmaPeLv5t");

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

#[program]
pub mod VerifierTest {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //test_addition_multiplication_equality(ctx);
        // Get the generator point in affine coordinates

        test_addition_multiplication_equality(ctx);
        Ok(())
    } 

    pub fn test_addition_multiplication_equality(ctx: Context<Initialize>) -> Result<()> {
        // Use the generator point for BN254 curve in little-endian format
        let bytes : [u8; 64] = [211, 207, 135, 109, 193, 8, 194, 211, 168, 28, 135, 22, 169, 22, 120, 217, 133, 21, 24, 104, 91, 4, 133, 155, 2, 26, 19, 46, 231, 68, 6, 3, 196, 162, 24, 90, 122, 191, 62, 255, 199, 143, 83, 227, 73, 164, 166, 104, 10, 156, 174, 178, 150, 95, 132, 231, 146, 124, 10, 14, 140, 115, 237, 21];
        // Prepare input for addition (G + G)
        let mut addition_input = Vec::new();
        addition_input.extend_from_slice(&bytes);
        addition_input.push(0u8); // Add the extra byte
        addition_input.extend_from_slice(&bytes);
        // Perform the addition
        msg!("Addition input: {:?}", addition_input);
        let addition_result = alt_bn128_addition(&addition_input).map_err(|e| {
            msg!("Addition error: {:?}", e);
            ErrorCode::CalculationError
        })?;
        msg!("Addition result: {:?}", addition_result);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum ErrorCode {
    #[msg("Error in elliptic curve calculation")]
    CalculationError,
    #[msg("Test failed")]
    TestFailed,
}