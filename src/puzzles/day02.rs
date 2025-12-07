pub fn solve() {
    let contents = std::fs::read_to_string("inputs/day02.txt").unwrap();

    let ranges = contents.split(',');

    let mut res1: u64 = 0;
    let mut res2: u64 = 0;
    for range in ranges {
        let parts: Vec<&str> = range.split("-").collect();
        let start: u64 = parts[0].parse::<u64>().unwrap();
        let end: u64 = parts[1].parse::<u64>().unwrap();
        
        res1 += eval_range(start, end);
        res2 += eval_range_p2(start, end);
    }

    println!("Part 1: {}", res1);
    println!("Part 2: {}", res2);
}

fn eval_range(start: u64, end: u64) -> u64 {
    let mut res = 0_u64;

    for n in start..=end {
        let digits = n.to_string();
        let digit_vec: Vec<&str> = digits.split("").collect();
        
        if digits.len() % 2 != 0 {
            continue;
        }

        let part0 = digit_vec[0..(digit_vec.len() / 2)].join("");
        let part1 = digit_vec[(digit_vec.len() / 2)..digit_vec.len()].join("");

        if part0 == part1 {
            println!("Invalid: {}", n);
            res += n;
        }

    }

    return res;
}


fn eval_range_p2(start: u64, end: u64) -> u64 {
    let mut res = 0_u64;

    for n in start..=end {
        let digits = n.to_string();
        let digit_vec: Vec<&str> = digits.split("").collect();
        
        if n < 10 {
            continue;
        }

        let mut matches = true;
        for divisor in divisors(digit_vec.len() - 2) {
            matches = true;
            let mut last: Option<String> = None;

            for i in (1..digit_vec.len()-divisor).step_by(divisor) {
                let part = digit_vec[i..i+divisor].join("");
                
                if let Some(prev) = last {
                    if prev != part {
                        matches = false;
                        break;
                    }
                }
                
                last = Some(part);
            }
            
            if matches {
                break;
            }
        }

        if matches {
            res += n;
        }

    }

    return res;
}

fn divisors(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 1..=(n/2) {
        if n % i == 0 {
            res.push(i);
        }
    }
    return res;
}