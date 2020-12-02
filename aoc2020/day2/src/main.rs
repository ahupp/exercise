use std::io::{self, BufRead};
use std::collections::HashSet;
use regex::Regex;

fn count_char(target : char, s: &str) -> i64 {
    let mut ct = 0;
    for c in s.chars() {
        if target == c {
            ct += 1;
        }
    }
    return ct;
}

fn main() {

    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let mut valid = 0;
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        for cap in re.captures_iter(&i) {
            let min : usize = cap[1].parse().unwrap();
            let max : usize = cap[2].parse().unwrap();
            let letter = &cap[3].chars().next().unwrap();
            let pass = &cap[4];

            let c1 = pass.chars().nth(min - 1).unwrap();
            let c2 = pass.chars().nth(max - 1).unwrap();
            if (c1 == *letter) ^ (c2 == *letter) {
                valid += 1;
            }
            break;
        }
    }
    println!("{}", valid);
}
