use primes::{PrimeSet, Sieve};
fn main() {
    let mut primes: Vec<i32> = Vec::new();
    let mut sumset: Vec<i32> = Vec::new();

    let mut i = 0;
    while i <= 1000 {
        let mut pset = Sieve::new();
        let (_ix, n) = pset.find(i);

        i = 0+&n+1;
        primes.push(n as i32);
    }

    let mut sum = 0;
    let mut i = 0;
    while (sum < 1000) {
        sumset.push(primes[i]);
        sum = &sum + primes[i];

        //println!("{} {}", primes[i], sum);
        if (sum > 1000) {
            sum = sum - primes[i];
            println!("X: {}", sum);
            println!("Y: {}", &i - 1);
            break;
        }
        i = &i + 1;
    }
}
