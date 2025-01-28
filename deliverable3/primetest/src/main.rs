use primes::{PrimeSet, Sieve};
fn main() {
    let para: i32 = 650;
    let mut primes: Vec<i32> = Vec::new();

    let mut i: i32 = 0;
    while i <= para {
        let mut pset = Sieve::new();
        let (_ix, n) = pset.find(i as u64);

        i = 0+&(n as i32)+1;
        primes.push(n as i32);
    }

    let mut sum: i32 = 0;
    for i in 3..24 {
        sum += primes[i];
    }

    println!("{}", sum);
}
