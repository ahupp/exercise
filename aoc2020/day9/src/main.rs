use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn is_valid(n: i64, preamble: &[i64]) -> bool {
    let mut pres = HashSet::new();
    for n in preamble {
        pres.insert(*n);
    }

    for i in preamble {
        let v = n - i;
        if preamble.contains(&v) && *i != v {
            return true;
        }
    }
    return false;
}

fn find_invalid(num: &Vec<i64>) -> i64 {
    let presize = 25;
    for i in presize..num.len() {
        let valid = is_valid(num[i], &num[i-presize..i]);
        if !valid {
            return num[i];
        }
    }
    panic!("derp");
}

fn main() {
    let mut num = Vec::new();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        let n : i64 = i.parse().unwrap();
        num.push(n);
    }

    let invalid = find_invalid(&num);

    let mut diffset = Vec::new();
    let mut run = 0;
    for i in &num {
        diffset.push(run);
        run += i;
    }
    diffset.push(run);

    println!("finding {}", invalid);
    for i in 0..diffset.len() {
        for j in (i+1)..diffset.len() {
            let s = diffset[j] - diffset[i];
            if s == invalid {
                let min = num[i..j].iter().min().unwrap();
                let max = num[i..j].iter().max().unwrap();
                println!("{} {} {} {} {}", *min + *max, min, max, i, j);
//                panic!("");
            }
        }
    }
}