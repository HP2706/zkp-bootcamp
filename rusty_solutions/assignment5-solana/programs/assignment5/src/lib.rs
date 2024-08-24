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




struct Verifier {}

impl Verifier {
    pub fn verify(A1: [u8; 64], B2: [u8; 64], C1: [u8; 64], x1: i32, x2: i32, x3: i32) -> Result<()> {
        
        let alpha_1 = 1;
        let beta_2 = 1;
        let gamma_2 = 1;
        let delta_2 = 1;
        Ok(())

    }

    fn play() -> Result<(),> {
        // Prepare input for addition (A1 + C1)
        let point = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
            33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
            49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
        ];

        // Prepare input for addition (point + point)
        let mut addition_input = Vec::new();
        addition_input.extend_from_slice(&point);
        addition_input.extend_from_slice(&point);

        // Perform the addition
        let addition_result = alt_bn128_addition(&addition_input).map_err(|_| error!(ErrorCode::CalculationError));

        // Prepare input for multiplication (point * 2)
        let mut multiplication_input = Vec::new();
        multiplication_input.extend_from_slice(&point);
        multiplication_input.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);

        // Perform the multiplication
        let multiplication_result = alt_bn128_multiplication(&multiplication_input).map_err(|_| error!(ErrorCode::CalculationError));

        // Compare results
        if addition_result == multiplication_result {
            msg!("Test passed: Addition result equals multiplication result");
        } else {
            msg!("Test failed: Addition result does not equal multiplication result");
            return Err(error!(ErrorCode::TestFailed));
        }

        Ok(())
    }

}


#[program]
pub mod VerifierTest {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing");
        msg!("test");
        
        
        msg!("Initialization complete");
        Ok(())
    }

    pub fn test_addition_multiplication_equality(ctx: Context<Initialize>) -> Result<()> {
        // Define a point on the curve (this is just an example, use a valid point in practice)
        let point = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
            33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
            49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
        ];

        // Prepare input for addition (point + point)
        let mut addition_input = Vec::new();
        addition_input.extend_from_slice(&point);
        addition_input.extend_from_slice(&point);

        // Perform the addition
        let addition_result = alt_bn128_addition(&addition_input).map_err(|_| error!(ErrorCode::CalculationError));
        msg!("addition result {:?}", addition_result);
        // Prepare input for multiplication (point * 2)
        let mut multiplication_input = Vec::new();
        multiplication_input.extend_from_slice(&point);
        multiplication_input.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);

        // Perform the multiplication
        let multiplication_result = alt_bn128_multiplication(&multiplication_input).map_err(|_| error!(ErrorCode::CalculationError));
        msg!("multiplication result {:?}", multiplication_result);
        // Compare results
        if addition_result == multiplication_result {
            msg!("Test passed: Addition result equals multiplication result");
        } else {
            msg!("Test failed: Addition result does not equal multiplication result");
            return Err(error!(ErrorCode::TestFailed));
        }

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