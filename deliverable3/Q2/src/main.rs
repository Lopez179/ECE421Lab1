use apint::{ApInt};
use primes::is_prime;

// Blue code is pseudo-code.
use rand::prelude::*; // Need this for rng.gen_range(…) function to work.
fn function(n: u32) -> u64 {
    
    let mut rng = rand::thread_rng();
    loop {
        let mut candidate = ApInt::from(rng.gen_range(0..n));
        let _ = candidate.set_bit_at(0);

        let value = ApInt::resize_to_u64(&candidate);

        if is_prime(value) == true {
            return value;
        }
    }
}

fn main() {
    let output = function(1000);
    println!("{}", output);
}