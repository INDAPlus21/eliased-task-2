use std::io::{self, BufRead};
use std::cmp;

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let secondn: i32 = nums[0];
        let firstn: i32 = nums[1];

        let halfn: i32 = if firstn % 2 == 0 {
            firstn / 2
        } else {
            (firstn + 1) / 2
        };

        // make this a function

        let halfsecondn: i32 = if secondn % 2 == 0 {
            secondn / 2
        } else {
            (secondn + 1) / 2
        };

        for column in 0..secondn {
            for row in 0..firstn {
                let shouldipushrow: i32;
                let shouldipushcolumn: i32;
                if row < halfn {
                    shouldipushrow = row + 1;
                } else {
                    shouldipushrow = firstn - row;
                }
                if column < halfsecondn {
                    shouldipushcolumn = column + 1;
                } else {
                    shouldipushcolumn = secondn - column;
                }
                let minimum : i32 = cmp::min(shouldipushrow, shouldipushcolumn);
                if minimum > 9 {
                    print!(".");
                } else {
                    print!("{}", minimum);
                }
            }
            println!();
        }
    }
}
