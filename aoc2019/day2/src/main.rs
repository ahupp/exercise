use std::io::{BufRead, self};

fn eval_bin<F : Fn(i32,i32)->i32>(mem : &mut Vec<i32>, pos : usize, fun : F) -> usize {
    let ret = fun(mem[mem[pos] as usize], mem[mem[pos+1] as usize]);
    let dest = mem[pos+2] as usize;
    mem[dest] = ret;
    pos + 3
}

// Evaluate the instruction at `pos`, and return the position of the next
// instruction
fn eval_at(mem : &mut Vec<i32>, pos : usize) -> usize {
    match mem[pos] {
        1 => eval_bin(mem, pos+1, |x,y| x + y),
        2 => eval_bin(mem, pos+1, |x,y| x * y),
        99 => mem.len(),
        _ => panic!("unknown opcode {}", mem[pos]),
    }
}

fn read_prog() -> Vec<i32> {
    let mut mem : Vec<i32> = Vec::new();
    for line in io::stdin().lock().lines() {
        for entry in line.unwrap().split(",") {
            let as_int : i32 = entry.parse().unwrap();
            mem.push(as_int);
        }
    }
    mem
}

fn run_prog(mut mem : &mut Vec<i32>) {
    let mut ip = 0;
    while ip < mem.len() {
        ip = eval_at(&mut mem, ip);
    }
}

fn main() {
    let mem = read_prog();
    for i in 1..100 {
        for j in 1..100 {
            let mut m = mem.clone();
            m[1] = i;
            m[2] = j;
            run_prog(&mut m);
            if m[0] == 19690720 {
                println!("{} {}", i, j);
                break;
            }
        }
    }
}
