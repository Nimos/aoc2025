use std::fs::File;
use std::io::{self, BufRead};


pub fn solve() {
    let path = "inputs/day01.txt";

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open {}: {}", path, e);
            return;
        }
    };

    let reader = io::BufReader::new(file);
    
    let mut dial = 50;
    let mut res1 = 0;
    let mut res2 = 0;
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {

                let mut chars = line.chars();
                let prefix = chars.next().unwrap();
                let rest: String = chars.collect();
                let num: i32 = rest.parse().unwrap();

                if prefix == 'L' {
                    if dial == 0 {
                        res2 -= 1;
                    }
                    dial -= num;
                    while dial < 0 {
                        dial = 100 + dial;
                        res2 += 1;
                    }
                    if dial == 0 {
                        res2 += 1;
                    }
                } else {
                    dial += num;
                    while dial > 99 {
                        dial = dial - 100;
                        res2 += 1
                    }
                }
                
                if dial == 0 {
                    res1 += 1;
                }
            }
            Err(e) => {
                eprintln!("Error reading a line: {}", e);
                return;
            }
        }
    }

    println!("Result pt 1: {}", res1);
    println!("Result pt 2: {}", res2);
}

