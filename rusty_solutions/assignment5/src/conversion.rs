use ark_bn254::{G1Affine, G2Affine, Fr, Fq, G1Projective};
use solana_program::alt_bn128::{self, prelude::*};
use solana_program::alt_bn128::{PodG1};
use ark_ec::{AffineRepr, CurveGroup, models::CurveConfig};
use ark_ff::{BigInteger, BigInteger256, Field, PrimeField, BigInt, Fp256};
use std::ops::{Mul, Add, Neg};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Read, Write, Flags, EmptyFlags, Validate};
use solana_program::alt_bn128::compression::prelude::{convert_endianness, G1, G2, alt_bn128_g1_decompress, alt_bn128_g2_decompress};
use solana_program::alt_bn128::prelude::{
    alt_bn128_addition, alt_bn128_multiplication, alt_bn128_pairing,
};

// Helper function to convert G1 point to bytes in the correct format
pub fn g1_to_bytes(point: &G1Affine) -> [u8; 64] {
    let mut bytes = [0u8; 64];
    let (x, y) = point.xy().unwrap();
    
    x.serialize_uncompressed(&mut bytes[0..32]).unwrap();
    y.serialize_uncompressed(&mut bytes[32..64]).unwrap();
    
    // Convert to big-endian
    bytes[0..32].reverse();
    bytes[32..64].reverse();
    
    bytes
}

pub fn bigint_to_bytes(bigint: BigInt<4>) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    
    BigInt::serialize_with_mode(&bigint, &mut bytes[..], Compress::Yes);
    bytes.reverse();
    bytes
}

pub fn scalar_to_bytes(scalar: u64) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[..8].copy_from_slice(&scalar.to_le_bytes());
    // Convert to big-endian
    bytes.reverse();
    bytes
}

pub fn g2_to_bytes(point: &G2Affine) -> [u8; 128] {
    let mut bytes = [0u8; 128];
    let (x, y) = point.xy().unwrap();
    
    x.serialize_uncompressed(&mut bytes[0..64]).unwrap();
    y.serialize_uncompressed(&mut bytes[64..128]).unwrap();
    
    // Convert to big-endian
    bytes[0..64].reverse();
    bytes[64..128].reverse();
    
    bytes
}