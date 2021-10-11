use std::io::{self, BufRead};
use std::collections::{HashMap};
use regex::Regex;

fn apply_mask(mask: &str, value: &str) -> Vec<u64> {
    let mut vc : Vec<char> = value.chars().collect();
    let mut initial = vec!['0'; 36 - vc.len()];
    initial.append(&mut vc);
    if initial.len() != mask.len() {
        panic!("len mismatch");
    }

    let mut ret = vec![initial];

    let set_all = |target: &mut Vec<Vec<char>>, i: usize, ch: char| {
        for out in target {
            out[i] = ch;
        }
    };

    for i in 0..mask.len() {
        let ch = mask.as_bytes()[i] as char;
        match ch {
            '0' => continue,
            '1' => {
                set_all(&mut ret, i, '1');
            }
            'X' => {
                set_all(&mut ret, i, '0');
                let mut cp = ret.clone();
                set_all(&mut cp, i, '1');
                ret.append(&mut cp);
            }
            _ => panic!("bad mask"),
        };
    }
    return ret.into_iter()
        .map(|vc| vc.into_iter().collect::<String>())
        .map(|s| {
            u64::from_str_radix(&s, 2).unwrap()
        })
        .collect();
}

fn main() {
    let mask_re = Regex::new(r"^mask = ([0-9X]+)").unwrap();
    let mem_re = Regex::new(r"mem\[(\d+)\] = ([0-9X]+)").unwrap();
    let mut mem = HashMap::new();
    let mut cur_mask = String::from_utf8(vec![b'X'; 36]).unwrap();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        if let Some(cap) = mask_re.captures(&i) {
            let ms = cap.get(1).unwrap().as_str();
            cur_mask = ms.to_owned();
            println!("mask = {}", cur_mask);
        } else if let Some(cap) = mem_re.captures(&i) {
            let addr: u64 = cap.get(1).unwrap().as_str().parse().unwrap();
            let value : u64 = cap.get(2).unwrap().as_str().parse().unwrap();
            let addr_str = format!("{:b}", addr);
            let addr_masked = apply_mask(&cur_mask, &addr_str);
            for i in addr_masked {
                mem.insert(i, value);
            }
        } else {
            panic!("bad line {}", i);
        }
    }

    let mut sum: u64 = 0;
    for (_, v) in &mem {
        sum += v;
    }
    println!("{}", sum);
}