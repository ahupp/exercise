use std::io::{self, BufRead};
use std::collections::{HashMap};


fn main() {
    let mut seed = Vec::new();
    let mut num_to_index = HashMap::new();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        for splat in i.split(',') {
            let n : usize = splat.parse().unwrap();
            num_to_index.insert(n, seed.len());
            seed.push((n, 0 as usize));
        }
        break;
    }

    while seed.len() <= 2020 {
        let end = seed.len() - 1;
        let (last_num, prev_that_num) = seed[end];
        let next_num = if prev_that_num == 0 {
            0
        } else {
            end - prev_that_num
        };

        let last_index = num_to_index.insert(next_num, seed.len()).unwrap_or(0 as usize);
        //println!("end={} ")
        println!("");
        dbg!(end + 1, next_num, last_index);
        seed.push((next_num, last_index));

//        println!("{}: last={} age={}", end, last, age);
    }
    println!("{}", seed[2019].0);
    println!("{}", seed[2020].0);
}