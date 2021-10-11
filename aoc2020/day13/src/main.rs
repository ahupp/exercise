use std::io::{self, BufRead};

fn find_closest(earliest: i64, sched: &Vec<i64>) -> (i64, i64) {
    for i in earliest.. {
        for s in sched {
            if i % s == 0 {
                return (*s, i - earliest);
            }
        }
    }
    panic!("not found");
}

fn is_ok(time: i64, sched: &Vec<(usize, i64)>) -> bool {
    for (i, v) in sched {
        if (time + *i as i64) % v == 0 {
            continue
        } else {
            return false;
        }
    }
    return true;
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let earliest : i64 = lines.next().unwrap().unwrap().parse().unwrap();
    let sched_line = lines.next().unwrap().unwrap();
    let mut sched : Vec<(usize, i64)> = Vec::new();
    let mut prod: i64 = 1;
    for (i, e) in sched_line.split(',').enumerate() {
        if e == "x" {
            continue;
        } else {
            let v = e.parse::<i64>().unwrap();
            if v - i as i64 > 0 {
                prod *= v - i as i64;
            }

            //prod *= v - i as u64;
            sched.push((i, v));
        }
    }

    /*
    x mod 37 == 0
    x + 27 mod 41 = 0
    */

    x*593 - 68 = y*433 - 37

    println!("{:?}", sched);
    let mut ct = 107692200;
    loop {
        if is_ok(ct, &sched) {
            println!("found {}", ct);
        }
        ct += 107692200;
    }

    return Ok(());
}