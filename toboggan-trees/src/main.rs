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

fn count_trees(over: usize, down: usize, rows: &Vec<String>) -> i64 {
    let width = rows[0].len();
    println!("{}", width);

    let mut trees = 0;
    let mut x = 0;
    for y in (0..(rows.len())).step_by(down) {
        println!("{}, {}, {}", y, x, rows[y].get(x%width..x%width+1).unwrap());
        if rows[y].get(x%width..x%width+1).unwrap() == "#" {
            println!("tree");
            trees += 1;
        }
        x += over;
    }
    return trees;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let rows = parse_file(&args[1]);

    let mut trees = count_trees(1, 1, &rows);
    trees *= count_trees(3, 1, &rows);
    trees *= count_trees(5, 1, &rows);
    trees *= count_trees(7, 1, &rows);
    trees *= count_trees(1, 2, &rows);


    println!("{}", trees);
}
