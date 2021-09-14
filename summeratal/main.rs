use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {

        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        if nums.iter().count() > 1 {
            let mut x = nums;
        
            let n = x.iter().count();
            let halfn = if n % 2 == 0 { n / 2 } else { (n + 1) / 2 };
            
            let mut numsummed = 0;
            let mut sum : i32 = 0;
        
            while numsummed < halfn {
                let mut templargest : i32 = 0;
                for i in x.clone() {
                    if i > templargest
                    {
                        templargest = i;
                    }
                }
                let index = x.iter().position(|z| *z == templargest).unwrap();
                x.remove(index);
                numsummed += 1;
                sum += templargest;
            }
        
            println!("{}", sum)
        } 
    }
}
