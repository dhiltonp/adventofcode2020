use std::env;
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use regex::Regex;
use std::ops::Index;

fn parse_file(file: &str) -> Vec<String> {
    let file = File::open(file).unwrap();
    let reader = BufReader::new(file);
    let mut strs = Vec::<String>::new();
    for line in reader.lines() {
        let l = line.unwrap();
        strs.push(l);
    }
    return strs;
}

fn validate_password(min:i32, max: i32, character: &str, password: &str) -> bool {
    let count: Vec<&str> = password.split(character).collect();
    let occurrences = (count.len() - 1) as i32;
    if !(occurrences >= min && occurrences <= max) {
        println!("not compliant: {}, {}, {}, {}", min, max, character, password);
    }

    return occurrences >= min && occurrences <= max;
}

fn valid_passwords(strs: Vec<String>) -> i32 {
    let parse_line = Regex::new("([0-9]+)-([0-9]+) ([^:]+): (.*)").unwrap();

    let mut count = 0;
    for s in &strs {
        println!("{}", s);
        let parsed = parse_line.captures(s).unwrap();
        let min = parsed.index(1).parse::<i32>().unwrap();
        let max = parsed.index(2).parse::<i32>().unwrap();
        let character = parsed.index(3);
        let password = parsed.index(4);
        if validate_password(min, max, character, password) {
            count += 1;
        }
    }
    return count;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let strs = parse_file(&args[1]);
    println!("{}", valid_passwords(strs))
}
