use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn parse_file(file: &str) -> Vec<i128> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    let mut nums = Vec::<i128>::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let n = l.parse::<i128>().unwrap();
        nums.push(n);
    }
    nums
}

fn find_numbers(nums: Vec<i128>) -> i128 {
    for x in 0..nums.len() {
        for y in (x+1)..nums.len() {
            // println!("{}, {}", x, y);
            for z in (y+1)..nums.len() {
                if nums[x]+nums[y]+nums[z] == 2020 {
                    // println!("{}, {}, {}", nums[x], nums[y], nums[z]);
                    return nums[x]*nums[y]*nums[z];
                }
            }
        }
    }
    0
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = parse_file(&args[1]);
    println!("{}", find_numbers(nums))
}
