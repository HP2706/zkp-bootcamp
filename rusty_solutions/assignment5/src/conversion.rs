use ark_bn254::{G1Affine, G2Affine, Fr, Fq, G1Projective};
use ark_ec::{AffineRepr, CurveGroup, models::CurveConfig};
use ark_ff::{BigInteger, BigInteger256, Field, PrimeField, BigInt, Fp256};
use std::ops::{Mul, Add, Neg};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Read, Write, Flags, EmptyFlags, Validate};
use solana_program::alt_bn128::compression::prelude::{convert_endianness, G1, G2, alt_bn128_g1_decompress, alt_bn128_g2_decompress};
use solana_program::alt_bn128::prelude::{
    alt_bn128_addition, alt_bn128_multiplication, alt_bn128_pairing,
};
use anyhow::Result;

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

pub fn bytes_to_g1(bytes: &[u8; 64]) -> Result<G1Affine> {
    if bytes.len() != 64 {
        return Err(anyhow::anyhow!(format!("Invalid byte length for scalar got {}", bytes.len())));
    }
    
    G1Affine::deserialize_with_mode(
        &*[&change_endianness(&bytes[0..64]), &[0u8][..]].concat(),
        Compress::No,
        Validate::Yes,
    )
    .map_err(|err| anyhow::anyhow!("Failed to deserialize G1 point: {}", err))
}

pub fn bytes_to_g2(bytes: &[u8; 128]) -> Result<G2Affine> {
    if bytes.len() != 128 {
        return Err(anyhow::anyhow!(format!("Invalid byte length for G2 point: expected 128, got {}", bytes.len())));
    }

    let changed_bytes = change_endianness(bytes);
    
    // Print the first few bytes for debugging
    println!("First 16 bytes of changed G2 input: {:?}", &changed_bytes[..16]);

    G2Affine::deserialize_with_mode(
        &*[&changed_bytes, &[0u8][..]].concat(),
        Compress::No,
        Validate::Yes,
    )
    .map_err(|err| {
        println!("Raw G2 input: {:?}", bytes);
        println!("Changed endianness G2 input: {:?}", changed_bytes);
        anyhow::anyhow!("Failed to deserialize G2 point: {}. Raw input: {:?}", err, bytes)
    })
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


pub fn change_endianness(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for b in bytes.chunks(32) {
        for byte in b.iter().rev() {
            vec.push(*byte);
        }
    }
    vec
}