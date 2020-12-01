use std::io::{self, BufRead};
use std::collections::HashSet;
fn main() {
    let mut num = HashSet::new();
    for i in io::stdin().lock().lines() {
        let s = i.unwrap();
        let i : i64 = s.parse().unwrap();
        num.insert(i);
    }

    for i in &num {
        for j in &num {
            let comp = 2020 - (i + j);
            if num.contains(&comp) {
                println!("{}", comp*i*j);
                return;
            }
        }
    }
}
