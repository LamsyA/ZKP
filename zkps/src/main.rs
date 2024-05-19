mod pedersen;
use num_bigint::{BigInt};
use pedersen::{PedersenCommitment, TPedersenCommitment,};

fn main() {
    let bits = 16;
    let newperdersen = PedersenCommitment::new(bits);
    println!("NewPerdersen: {}", newperdersen);
    let m = BigInt::from(5);
    let r = BigInt::from(3);
    let commitment = newperdersen.commit(&m, &r);
    println!("Commitment: {}", commitment);
    let verify = newperdersen.verify(&commitment, &m, &r);
    println!("Verify: {}", verify);


}