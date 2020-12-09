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

fn is_sum(n: i128, nums: Vec<i128>) -> bool{
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            println!("{} = {}+{}", n, nums[i], nums[j]);
            if n == nums[i]+nums[j] {
                println!("true");
                return true;
            }
        }
    }
    false
}

fn find_invalid(preamble: usize, nums: &[i128]) -> i128 {
    for i in preamble..nums.len() {
        if is_sum(nums[i], nums[i-preamble..i].to_owned()) {
            continue;
        } else {
            return i as i128;
        }
    }

    0
}

fn find_contiguous(val: i128, nums: &[i128]) {
    for i in 0..nums.len() {
        let mut sum = nums[i];
        for j in i+1..nums.len() {
            sum += nums[j];
            if sum == val {
                let mut min = nums[i];
                let mut max = nums[i];
                for num in nums.iter().take(j).skip(i) {
                    println!("min/max for {}", num);
                    if min > *num {
                        min = *num;
                    }
                    if max < *num {
                        max = *num;
                    }
                }
                println!("{}-{}, {}", i, j, min+max);
            }
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = parse_file(&args[1]);
    let invalid = find_invalid(25, &nums);
    println!("{}: {}", invalid, nums[invalid as usize]);

    find_contiguous(nums[invalid as usize], &nums);
}
