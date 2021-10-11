use std::io::{self, BufRead};

fn main() {

    let mut wx = 10;
    let mut wy = 1;

    let rotate = |deg: i64, wx: i64, wy: i64| {
        let mut n = deg / 90;
        let mut wx = wx;
        let mut wy = wy;

        loop {
            if n == 0 {
                return (wx, wy);
            }
            let tx = wx;
            let ty = wy;

            if n > 0 {
                n -= 1;
                wx = ty;
                wy = -tx;
            } else {
                n += 1;
                wx = -ty;
                wy = tx;
            }
        }
    };

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        let c = i.chars().nth(0).unwrap();
        let num: i64 = i[1..i.len()].parse().unwrap();
        match c {
            'R' => {
                let (tx, ty) = rotate(num, wx, wy);
                wx = tx;
                wy = ty;
            },
            'L' => {
                let (tx, ty) = rotate(-num, wx, wy);
                wx = tx;
                wy = ty;
            },
            'N' => {
                wy += num;
            },
            'S' => {
                wy -= num;
            },
            'E' => {
                wx += num;
            },
            'W' => {
                wx -= num;
            },
            'F' => {
                x += wx*num;
                y += wy*num;
            }
            _ => panic!("unknown char {}", c),
        }
        //println!();
        println!("{} -> {} {} {} {}", i, wx, wy, x, y);
    }
    println!("{}", x.abs() + y.abs());
}
