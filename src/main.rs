use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let _perfect_numbers: Vec<i32> = Vec::new();
    for i in 1..100000000 {
        let mut divisors: Vec<i32> = Vec::new();
        for j in 1..i {
            if i % j == 0 {
                divisors.push(j);
            }
        }
        let sum_divisors: i32 = divisors.iter().sum();
        if sum_divisors == i {
            println!("{} is a perfect number", i);
            let duration = start.elapsed();
            println!("Found in: {:?}", duration);
        }
    }
}
