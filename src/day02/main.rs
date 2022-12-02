use std::fs;
// use std::io;
use std::io::{self, BufRead};
use std::str::SplitWhitespace;


fn main() {
    let file = fs::File::open("./src/day02/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    // part 1
    let mut result = 0;
    for line in lines {
        if let Ok(tmp) = line {
            // let x = tmp.split_whitespace().collect::<Vec<_>>();
            // let x = tmp.split_whitespace();
            match tmp.as_str() {
                "A X" => { result += 1 + 3 }
                "B X" => { result += 1 + 0 }
                "C X" => { result += 1 + 6 }
                "A Y" => { result += 2 + 6 }
                "B Y" => { result += 2 + 3}
                "C Y" => { result += 2 + 0}
                "A Z" => { result += 3 + 0}
                "B Z" => { result += 3 + 6}
                "C Z" => { result += 3 + 3 }
                _ => panic!()
            }
        }

        // for cc in x {
        //
        // }
    }

    // part2
    let file2 = fs::File::open("./src/day02/input.txt").unwrap();
    let lines2 = io::BufReader::new(file2).lines();
    let mut result2 = 0;
    for line in lines2 {
        if let Ok(tmp) = line {
            // let x = tmp.split_whitespace().collect::<Vec<_>>();
            // let x = tmp.split_whitespace();
            match tmp.as_str() {
                // lose
                "A X" => { result2 += 3 + 0 }
                "B X" => { result2 += 1 + 0 }
                "C X" => { result2 += 2 + 0 }
                // draw
                "A Y" => { result2 += 1 + 3 }
                "B Y" => { result2 += 2 + 3 }
                "C Y" => { result2 += 3 + 3 }
                // win
                "A Z" => { result2 += 2 + 6 }
                "B Z" => { result2 += 3 + 6 }
                "C Z" => { result2 += 1 + 6 }
                _ => panic!()
            }
        }

        // for cc in x {
        //
        // }
    }
    println!("part 2: {}", result2);

    // let file = fs::read_to_string("./input.txt").unwrap();
}
