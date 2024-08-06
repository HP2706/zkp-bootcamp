//secp256k1
use generic_ec::{coords::HasAffineXY, curves::Secp256k1, Curve, Point, Scalar, SecretScalar};
use rand::thread_rng;
use tiny_keccak::{Hasher, Keccak};

fn hash_message(
    message: &str
) -> Scalar<Secp256k1> {
    let mut hasher = Keccak::v256();
    hasher.update(message.as_bytes());
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);
    Scalar::<Secp256k1>::from_be_bytes(&hash).unwrap()
}

fn sign_message(
    message: &str, 
    private_key: SecretScalar<Secp256k1>
) -> (Scalar<Secp256k1>, Scalar<Secp256k1>) {
    //random nonce k
    let mut rng = thread_rng();
    let k = Scalar::<Secp256k1>::random(&mut rng);
    let curve_point = Point::<Secp256k1>::generator() * k;
    let hash = hash_message(message);
    let r = curve_point.coords().unwrap().x.to_scalar();
    let mod_k_inv = k.invert().unwrap();
    //s = (mod_k_inv * (h + r * pv_key.d)) 
    let s = (mod_k_inv * (hash + r * private_key));
    (r, s)
}

fn verify_signature(
    message: &str,
    signature: (Scalar<Secp256k1>, Scalar<Secp256k1>),
    public_key: Point<Secp256k1>
) -> bool {
    let hash = hash_message(message);
    let (r, s) = signature;
    let mod_s_inv = s.invert().unwrap();
    //curve_point = cv.add_point(cv.mul_point(h * s1, cv.generator), cv.mul_point(r * s1, public_key.W))
    let curve_point = Point::<Secp256k1>::generator() * (hash * mod_s_inv) + (public_key * r * mod_s_inv);
    let r_prime = curve_point.coords().unwrap().x.to_scalar();
    r_prime == r
}


pub fn check_exercises(){
    //setup
    let message = "hello world";
    let private_key = SecretScalar::<Secp256k1>::from_be_bytes(&[
        0xac, 0x09, 0x74, 0xbe, 0xc3, 0x9a, 0x17, 0xe3,
        0x6b, 0xa4, 0xa6, 0xb4, 0xd2, 0x38, 0xff, 0x94,
        0x4b, 0xac, 0xb4, 0x78, 0xcb, 0xed, 0x5e, 0xfc,
        0xae, 0x78, 0x4d, 0x7b, 0xf4, 0xf2, 0xff, 0x80
    ]).unwrap();

    let public_key =  Point::<Secp256k1>::generator() * &private_key;
    let signature = sign_message(message, private_key);
    let is_valid = verify_signature(message, signature, public_key);
    println!("Is signature valid? {}", is_valid);
}