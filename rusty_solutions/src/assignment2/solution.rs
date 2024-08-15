use num_bigint::BigInt;
use num_bigint::BigUint;
use num_traits::{One, Zero, ToPrimitive};
use num_rational::Ratio;

//a different integer library 
use rug::Integer;
use discrete_logarithm::discrete_log;

fn exercise_1() {
    let n = BigInt::from(9551);
    let g = BigInt::from(5);
    let encrypted_number = BigInt::from(5666);
    
    for s in 0..9551 {
        if g.modpow(&BigInt::from(s), &n) == encrypted_number {
            assert!(g.modpow(&BigInt::from(s), &n) == encrypted_number);
            println!("s = {}", s);
            return;
        }
    }
    panic!("No solution found");
}

fn exercise_2() {
    let n = Integer::from(1000004119);
    let g = Integer::from(5);
    let encrypted_number = Integer::from(767805982);

    let student_solution = discrete_log(&n, &encrypted_number, &g);
    match student_solution {
        Ok(solution) => println!("student_solution is {}", solution),
        Err(e) => println!("Error: {}", e),
    }
}



fn exercise_4() {
    let n = BigUint::parse_bytes(b"1000004119", 10).unwrap(); // large prime
    let g = BigUint::from(5u32); // generator
    let x_secret = BigUint::from(420u32);
    let y_secret = BigUint::from(888u32);

    let prover1_x = g.modpow(&(BigUint::from(2u32) * &x_secret), &n);
    let prover1_y = g.modpow(&(BigUint::from(8u32) * &y_secret), &n);
    let prover2_x = g.modpow(&(BigUint::from(5u32) * &x_secret), &n);
    let prover2_y = g.modpow(&(BigUint::from(3u32) * &y_secret), &n);


    fn verifier(elm_1: &BigUint, elm_2: &BigUint, base: &BigUint, n: &BigUint, target_value: &BigUint) -> bool {
        // as the verifier only sees
        // elm_1 = g^(2*x_secret) % n, elm_2 = g^(8*y_secret) % n and not the actual solution
        (elm_1 * elm_2) % n == base.modpow(target_value, n)
    }

    println!(
        "verifier(prover1_x, prover1_y, g, n, 7944) = {}",
        verifier(&prover1_x, &prover1_y, &g, &n, &BigUint::from(7944u32))
    );
    println!(
        "verifier(prover2_x, prover2_y, g, n, 4764) = {}",
        verifier(&prover2_x, &prover2_y, &g, &n, &BigUint::from(4764u32))
    );
}

fn mod_inverse(a: &BigInt, m: &BigInt) -> BigInt {
    let mut t = BigInt::zero();
    let mut newt = BigInt::one();
    let mut r = m.clone();
    let mut newr = a.clone();

    while !newr.is_zero() {
        let quotient = &r / &newr;
        (t, newt) = (newt.clone(), t - &quotient * &newt);
        (r, newr) = (newr.clone(), r - &quotient * &newr);
    }

    if r > BigInt::one() {
        panic!("a is not invertible");
    }
    if t < BigInt::zero() {
        t += m;
    }
    t
}


fn exercise_5() {
    let p = BigInt::from(1033); // our prime

    let inv_192 = mod_inverse(&BigInt::from(192), &p);
    let inv_511 = mod_inverse(&BigInt::from(511), &p);

    let term1 = (&BigInt::from(53) * &inv_192) % &p;
    let term2 = (&BigInt::from(61) * &inv_511) % &p;

    let result = (&term1 + &term2) % &p;

    println!("53/192 + 61/511 (mod {}) = {}", p, result);

    let actual_sum = Ratio::new(BigInt::from(53), BigInt::from(192)) + 
                     Ratio::new(BigInt::from(61), BigInt::from(511));
    println!("Actual sum: {}", actual_sum);
    println!("Decimal approximation: {:.6}", actual_sum.to_f64().unwrap());
}

pub fn check_exercises() {
    println!("Exercise 1\n");
    exercise_1();
    println!("\nExercise 2\n");
    exercise_2();
    println!("\nExercise 4\n");
    exercise_4();
    println!("\nExercise 5\n");
    exercise_5();
}