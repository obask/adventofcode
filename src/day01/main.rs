use std::cmp::max;
use std::fs;
// use std::io;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::str::SplitWhitespace;


fn main() {
    let file = fs::File::open("./src/day01/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    // part 1
    let mut mx = 0;
    let mut result = 0;
    for line in lines {
        if let Ok(tmp) = line {
            match tmp.parse::<u32>() {
                Ok(num) => {
                    result += num
                }
                Err(_) => {
                    mx = max(mx, result);
                    result = 0;
                }
            }
        }
        // for cc in x {
        //
        // }
    }
    mx = max(mx, result);
    println!("part 1: {}", mx);

    // part2
    let file2 = fs::File::open("./src/day01/input.txt").unwrap();
    let lines2 = io::BufReader::new(file2).lines();
    let mut result2 = 0;
    let mut mxx = Vec::new();
    for line in lines2 {
        if let Ok(tmp) = line {
            match tmp.parse::<u32>() {
                Ok(num) => {
                    result2 += num
                }
                Err(_) => {
                    mxx.push(result2);
                    result2 = 0;
                }
            }
        }
    }
    mxx.push(result2);
    mxx.sort();

    println!("part 2: {}", mxx[mxx.len()-1]
     + mxx[mxx.len()-2]
    + mxx[mxx.len()-3]);

    // let file = fs::read_to_string("./input.txt").unwrap();
}
