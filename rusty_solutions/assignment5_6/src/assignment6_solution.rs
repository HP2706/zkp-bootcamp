use ark_bn254::{G1Affine, G2Affine, Fr, Fq, G1Projective, Config};
use ark_ff::BigInt;
use ark_ec::{AffineRepr, CurveGroup, bn::BnConfig};
use std::ops::Add;
use crate::conversion::{g1_to_bytes, scalar_to_bytes, g2_to_bytes, bigint_to_bytes, bytes_to_g1};
use ark_ec::pairing::Pairing;
use ark_ec::pairing::PairingOutput;
use ark_ec::models::short_weierstrass::{Affine};
use ark_bn254::{Fq12Config, Bn254};
use ark_bn254::g1::Config as G1Config;
use ark_bn254::g2::Config as G2Config;


fn vecs(a : Vec<impl AffineRepr>) -> Vec<impl AffineRepr> {
    a.into_iter().map(|p| p).collect()
}

#[derive(Clone, Copy)]
enum CurvePoint {
    G1(G1Affine),
    G2(G2Affine),
}



pub struct Matrix<T: Copy> {
    pub n_rows: usize,
    pub n_cols: usize,
    pub data: Vec<T>,
}

impl<T: Copy> Matrix<T> {

    pub fn index(&self, row: usize, col: usize) -> T {
        self.data[row * self.n_cols + col]
    }
}

fn matrix_point_dot_prod(
    M: Matrix<Fr>, 
    s: Matrix<impl AffineRepr>
) -> Matrix<CurvePoint> {
    assert!(M.n_cols == s.n_rows);
    let mut out = Vec::new();
    for i in 0..M.n_rows {
        let mut buf : Option<impl AffineRepr> = None;
        for j in 0..M.n_cols {
            match buf {
                Some(p) => {
                    let elm = p.mul_bigint(BigInt::from(M.index(i, j)));
                    buf = Some(buf.unwrap().add(elm));
                },
                None => {
                    buf = Some(s.index(i, j).mul_bigint(BigInt::from(M.index(i, j))))
                }
            }
        }

        out.push(CurvePoint::G2(buf));
    }

    Matrix {
        n_rows: M.n_rows,
        n_cols: s.n_cols,
        data: out,
    }
}

fn hadamard_prod(
    a: Matrix<CurvePoint>, 
    b: Matrix<CurvePoint>
) -> Vec<PairingOutput<ark_bn254::Bn254>> {
    assert_eq!(a.n_rows, b.n_rows);
    assert_eq!(a.n_cols, b.n_cols);
    
    a.data.iter().zip(b.data.iter()).map(|(a_point, b_point)| {
        match (a_point, b_point) {
            (CurvePoint::G1(p1), CurvePoint::G2(p2)) => ark_bn254::Bn254::pairing(p1.into_group(), p2.into_group()),
            (CurvePoint::G2(p2), CurvePoint::G1(p1)) => ark_bn254::Bn254::pairing(p1.into_group(), p2.into_group()),
            _ => panic!("Invalid point combination"),
        }
    }).collect()
}

pub fn check_it_works() {
    let L = Matrix {
        n_rows: 3,
        n_cols: 3,
        data: vec![Fr::from(1), Fr::from(0), Fr::from(0), 
                   Fr::from(0), Fr::from(1), Fr::from(0), 
                   Fr::from(0), Fr::from(0), Fr::from(1)],
    };

    let G1 = G1Affine::generator();
    let G2 = G2Affine::generator();

    let R = Matrix {
        n_rows: 3,
        n_cols: 3,
        data: vec![Fr::from(1), Fr::from(0), Fr::from(0), 
                   Fr::from(0), Fr::from(1), Fr::from(0), 
                   Fr::from(0), Fr::from(0), Fr::from(1)],
    };

    let S = Matrix {
        n_rows: 3,
        n_cols: 1,
        data: vec![CurvePoint::G1(G1), 
                   CurvePoint::G2(G2), 
                   CurvePoint::G1(G1)],
    };

    let S2 = Matrix {
        n_rows: 3,
        n_cols: 1,
        data: vec![CurvePoint::G2(G2), 
                   CurvePoint::G1(G1), 
                   CurvePoint::G2(G2)],
    };


    let p = G1Affine::generator();
    let q = G2Affine::generator();
    let pairing : PairingOutput<Bn254> = Pairing::pairing(p.into_group(), q.into_group());
    println!("{:?}", pairing);

    let O = matrix_point_dot_prod(L, S2);
    let P = matrix_point_dot_prod(R, S);
    for (a, b) in O.data.iter().zip(P.data.iter()) {
        println!("a: {:?}", a.get_type());
        println!("b: {:?}", b.get_type());
    }

    
    //let res = hadamard_prod(O, P);
    /*
    let res = hadamard_prod(O, P);
    println!("{:?}", res);
    println!("{:?}", res.len()); */
}
