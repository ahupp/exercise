use std::io::{BufRead, self};
use std::collections::HashMap;

#[derive(Debug)]
enum Dir {
    Right, Left, Up, Down
}

fn parse_step(step : &str) -> (Dir, i32) {
    let rest = &step[1..];
    let rest_i : i32 = rest.parse().unwrap();
    let dir = step.chars().next().unwrap();
    let ch = match dir {
        'R' => Dir::Right,
        'L' => Dir::Left,
        'U' => Dir::Up,
        'D' => Dir::Down,
        _ => panic!("unknown direction {}", dir),
    };
    (ch, rest_i)
}

fn parse_wire(line : &str) -> Vec<(Dir, i32)> {
    line.split(",").map(parse_step).collect()
}

fn read_wires() -> Vec<Vec<(Dir,i32)>> {
    io::stdin().lock().lines().map(|x| parse_wire(&x.unwrap())).collect()
}

// expand steps (like "Up 25 steps") into the full set of coordinats that step
// covers
fn expand_wire(wire : &Vec<(Dir,i32)>) -> HashMap<(i32, i32), i32> {
    let mut ret = HashMap::new();
    let mut cx = 0;
    let mut cy = 0;
    let mut steps = 0;
    for i in wire {
        let (dir, amount) = i;
        let (dx, dy) = match dir {
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };
        for _ in 1..(amount+1) {
           cx += dx;
           cy += dy;
           steps += 1;
           ret.insert((cx, cy), steps);
        }
    }
    ret
}


fn main() {
    let wires = read_wires();
    assert!(wires.len() == 2, "wrong # of wires");
    let a = expand_wire(&wires[0]);
    let b = expand_wire(&wires[1]);

    let mut candidate = Vec::new();
    for (k,v1) in a.iter() {
        let sum = match b.get(k) {
            // combined path length at crossing
            Some(v2) => v2 + v1,
            None => continue,
        };
        candidate.push(sum);
    }
    candidate.sort();
    println!("{:?}", candidate);
}

