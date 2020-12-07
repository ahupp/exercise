use std::io::{self, BufRead};
use std::collections::HashMap;

fn all_yes(group: &Vec<String>) -> usize {
    let mut yes = HashMap::new();
    for g in group {
        for letter in g.chars() {
            let value = yes.entry(letter).or_insert(0);
            *value += 1 as usize;
        }
    }

    yes.retain(|_, v| *v == group.len());

    return yes.len();
}

fn main() {
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        if i.trim().len() == 0 {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(i);
        }
    }
    if group.len() > 0 {
        groups.push(group);
    }

    let total : usize = groups.iter().map(all_yes).sum();
    println!("{}", total);
}

