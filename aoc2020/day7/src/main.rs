use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn de_s<'a>(inp: &'a str) -> &'a str {
    if inp.chars().nth_back(0).unwrap() == 's' {
        &inp[0..inp.len() - 1]
    } else {
        inp
    }
}

fn main() {
    let mut tree = HashMap::new();
    let re_outer = Regex::new(r"^(\w+ \w+ \w+) contain (.*)\.$").unwrap();
    let trim_num = Regex::new(r"^(\d+) (.*)$").unwrap();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        for c in re_outer.captures(&i) {
            let head = de_s(&c[1]);
            let tail = &c[2];
            println!("{} {}", head, tail);
            if tail.trim() == "no other bags" {
                continue;
            }

            for j in tail.split(",") {
                for c2 in trim_num.captures_iter(j.trim()) {
                    let num : u32 = c2[1].parse().unwrap();
                    let contained = de_s(&c2[2]);
                    tree.entry(head.to_owned())
                        .or_insert_with(|| Vec::new()).push((num, contained.to_owned()));
                }
            }

            break;
        }
    }

    //let mut result : HashSet<&str> = HashSet::new();
    let kids = find_all("shiny gold bag", &tree);
    println!("{}", kids);
}

fn find_all<'a>(which: &str, tree: &'a HashMap<String, Vec<(u32, String)>>) -> u32 {
    let entry = tree.get(which);
    let mut res: u32 = 1;
    if let Some(parents) = entry {
        for (num, color) in parents {
            res += num * find_all(color, tree) ;
        }
    }
    return res;
}
