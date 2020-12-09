use std::fs::File;
use std::env;
use std::io::{Read, BufReader, BufRead};

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let rows = parse_file(&args[1]);
    let width = rows[0].len();
    println!("{}", width);

    let mut trees = 0;
    let mut x = 0;
    for y in (0..(rows.len())).step_by(1) {
        println!("{}, {}, {}", y, x, rows[y].get(x%width..x%width+1).unwrap());
        if rows[y].get(x%width..x%width+1).unwrap() == "#" {
            println!("tree");
            trees += 1;
        }
        x += 3;
    }
    println!("{}", trees);
}
