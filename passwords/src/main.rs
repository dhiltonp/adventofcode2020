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

fn validate_password(min:usize, max: usize, character: &str, password: &str) -> bool {
    let count: Vec<&str> = password.split(character).collect();
    let occurrences = (count.len() - 1);
    let first = password.get(min-1..min).unwrap();
    let second = password.get(max-1..max).unwrap();
    println!("{}, {}, {}", first, second , (first == character) ^ (second == character));
    
    return (first == character) ^ (second == character);
}

fn valid_passwords(strs: Vec<String>) -> i32 {
    let parse_line = Regex::new("([0-9]+)-([0-9]+) ([^:]+): (.*)").unwrap();

    let mut count = 0;
    for s in &strs {
        println!("{}", s);
        let parsed = parse_line.captures(s).unwrap();
        let min = parsed.index(1).parse::<usize>().unwrap();
        let max = parsed.index(2).parse::<usize>().unwrap();
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
