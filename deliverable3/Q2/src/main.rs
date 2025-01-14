// Blue code is pseudo-code.
use rand::prelude::*; // Need this for rng.gen_range(…) function to work.
fn function(n: u32) -> Int {
    let mut rng = rand::thread_rng();
    loop {
        let mut candidate::Int = Int::from(rng.gen_range(0, n));
        candidate.set_bit(0, true);
        let i = u64::from(&candidate);
        if is_prime(i)== true {
            return candidate;
        }
    }
}