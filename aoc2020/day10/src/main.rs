use std::io::{self, BufRead};

fn adapter_chain(start_index: usize, chain: &Vec<i64>, memo: &mut Vec<Option<u64>>) -> u64 {
      if let Some(value) = memo[start_index] {
        println!("memo hit {} {}", value, start_index);
        return value;
    }

    let mut uniq = 0;
    let mut i = start_index + 1;

    loop {
        let delta = chain[i] - chain[start_index];
        let is_compat = delta <= 3;
        if !is_compat {
            break;
        }
        if i == chain.len() - 1 {
            // found a valid chain
            uniq += 1;
            break;
        }
        uniq += adapter_chain(i, chain, memo);
        i += 1;
    }

    memo[start_index] = Some(uniq);
    println!("memo miss {} {}", uniq, start_index);
    return uniq;
}

fn main() {
    let mut adapt = Vec::new();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        let n : i64 = i.parse().unwrap();
        adapt.push(n);
    }
    adapt.push(0);
    let maxin = adapt.iter().max().unwrap() + 3;
    adapt.sort();
    adapt.push(maxin);

    let mut memo = Vec::new();
    memo.resize(adapt.len(), None);
    let uniq = adapter_chain(0, &adapt, &mut memo);
    println!("{}", uniq);
}