use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i64> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let integer1 = nums[0];
        let integer2 = nums[1];
        let difference: i64 = integer1-integer2;
        println!("{}", difference.abs());
    }
}
