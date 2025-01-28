use core::num;
use std::{process::Output, usize};

use primes::{PrimeSet, Sieve, is_prime};

fn get_largest_primal_sum(primes: &Vec<i32>, mut sum: i32, leftbound_r: &usize, rightbound_r: &usize) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    
    let mut rightbound = *rightbound_r;
    let leftbound = *leftbound_r;

    let mut leftmark = leftbound;
    let mut rightmark = rightbound;

    let mut primal_sum = 0;
    while sum + primes[rightbound]< 1000 {
        rightbound = rightbound + 1;
        sum = sum + primes[rightbound];
        if is_prime(sum as u64) {
            leftmark = leftbound;
            rightmark = rightbound;
            primal_sum = sum;
        }
    }
    output.push(leftmark as i32);
    output.push(rightmark as i32);
    output.push(primal_sum);
    output
}
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

    let mut leftbound: usize = 0;
    let mut rightbound: usize = 0;
    let mut leftmark: usize = 0;
    let mut rightmark: usize = 0;
    let sum: i32 = 2;

    let mut primal_sum: i32 = 0;
    let mut number_of_terms: usize = 0;
    
    let mut current_solution = get_largest_primal_sum(&primes, sum, &leftbound, &rightbound);
    leftmark = current_solution[0] as usize;
    rightmark = current_solution[1] as usize;
    primal_sum = current_solution[2];
    let mut new_sum = primal_sum;

    leftbound = leftmark as usize;
    rightbound = rightmark as usize;
    while (rightbound < primes.len()) && (rightbound > leftbound)  {
        
        new_sum = new_sum - primes[leftbound];
        leftbound += 1;

        current_solution = get_largest_primal_sum(&primes, new_sum, &leftbound, &rightbound);
        if (current_solution[2] != 0) && (current_solution[1] - current_solution[0] + 1 > number_of_terms as i32) {
            
            leftbound = current_solution[0] as usize;
            rightbound = current_solution[1] as usize;
            leftmark = leftbound;
            rightmark = rightbound;
            new_sum = current_solution[2];
            primal_sum = new_sum;
            number_of_terms = rightbound - leftbound + 1;
        }
    }
    
    let mut ouput_vec: Vec<i32> = Vec::new();
    for i in leftmark..rightmark+1 {
        ouput_vec.push(primes[i]);
    }

    println!("{:?}", ouput_vec);
    println!("Y: {}, {} terms", primal_sum, number_of_terms);

}
