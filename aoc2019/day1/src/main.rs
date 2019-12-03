use std::io::{self, BufRead};
use std::cmp;

// https://adventofcode.com/2019/day/1

// Because of rounding you get different results when calculating the recursive
// fuel costs per-module or once at the end.  The "correct" answer requires this
// per-module, but I believe that's undercounting in a real system.
fn weight_to_fuel(weight : i64) -> i64 {
    let mut fuel = cmp::max(weight / 3 - 2, 0);
    // how much fuel to carry this fuel?
    if fuel > 0 {
        fuel += weight_to_fuel(fuel);
    }
    fuel
}

fn main() {
    let mut fuel = 0;
    for i in io::stdin().lock().lines() {
        let s = i.unwrap();
        let i : i64 = s.parse().unwrap();
        fuel += weight_to_fuel(i);
    }

    println!("total fuel: {}", fuel);
}
