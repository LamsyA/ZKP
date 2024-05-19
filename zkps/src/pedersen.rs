use num_bigint::{BigInt, RandBigInt};
use num_primes::Generator;
use num_traits::{One, Zero};
use rand::thread_rng;
use std::fmt;


#[derive(Debug, PartialEq)]
pub struct PedersenCommitment {
    g: BigInt,
    h: BigInt,
    p: BigInt,
    q: BigInt,
}


pub trait TPedersenCommitment {
     fn new(bits: usize) -> Self;

    fn commit(&self, m: &BigInt, r: &BigInt) -> BigInt;

    fn verify(&self, c: &BigInt, m: &BigInt, r: &BigInt) -> bool;
}


impl TPedersenCommitment for PedersenCommitment {
     fn new(bits: usize) -> Self {
        let mut rng = thread_rng();

        // Generate a large safe prime q (this is BigUint)
        let q_biguint = Generator::safe_prime(bits);
        println!("q (BigUint) = {}", q_biguint);

        // Convert BigUint to BigInt
        let q = BigInt::from_bytes_le(num_bigint::Sign::Plus, &q_biguint.to_bytes_le());
        println!("q (BigInt) = {}", q);

        // Compute p = 2q + 1
        let two = BigInt::from(2);
        let p = &q * &two + BigInt::one();
        println!("p = {}", p);

        // Choose generators g and h
        let g = rng.gen_bigint_range(&BigInt::one(), &p);
        println!("g = {}", g);
        let h = rng.gen_bigint_range(&BigInt::one(), &p);
        println!("h = {}", h);

        PedersenCommitment { g, h, p, q }
    }

    fn commit(&self, m: &BigInt, r: &BigInt) -> BigInt {
        let gm = self.g.modpow(m, &self.p);
        let hr = self.h.modpow(r, &self.p);
        (gm * hr) % &self.p
    }

    fn verify(&self, c: &BigInt, m: &BigInt, r: &BigInt) -> bool {
        let commitment = self.commit(m, r);
        commitment == *c
    }
}

impl fmt::Display for PedersenCommitment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data field.
        write!(f, "PedersenCommitment(g: {}, h: {}, p: {}, q: {})", self.g, self.h, self.p, self.q)
    }
}

pub fn main() {
    let bits = 16; // Bit length for the prime q
    let pedersen = PedersenCommitment::new(bits);

    let mut rng = thread_rng();

    // Choose a random message m and random value r
    let m = rng.gen_bigint_range(&BigInt::zero(), &pedersen.q);
    println!("Message: {}", m);
    let r = rng.gen_bigint_range(&BigInt::zero(), &pedersen.q);
    println!("random value: {}", r);
    let v = rng.gen_bigint_range(&BigInt::zero(), &pedersen.q);
    println!("another random value: {}", v);

    // Create a commitment
    let commitment = pedersen.commit(&m, &r);
    println!("Commitment: {}", commitment);

    // Verify the commitment
    let is_valid = pedersen.verify(&commitment, &m, &r);
    println!("Is valid: {}", is_valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pedersen_commitment() {
        let pedersen = PedersenCommitment::new(16);
        let m = BigInt::from(5);
        let r = BigInt::from(3);
        let commitment = pedersen.commit(&m, &r);
        assert_eq!(pedersen.verify(&commitment, &m, &r), true);
    }
}
