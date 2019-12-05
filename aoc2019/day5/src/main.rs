use std::io::{BufRead, self, BufReader};
use std::env;
use std::fs::File;

enum Value {
    Lookup(i32),
    Immediate(i32),
}

enum Inst {
    Unary(Value),
    Binary(Value, Value, Value),    
}

fn lookup(mem: &Vec<i32>, pos: usize, immediate: bool) -> i32 {
    if immediate {
        mem[pos]
    } else {
        mem[mem[pos] as usize]
    }
}

fn eval_bin<F : Fn(i32,i32)->i32>(mem : &mut Vec<i32>, pos : usize, fun : F, a_mode: bool, b_mode: bool) -> usize {
    let ret = fun(lookup(mem, pos, a_mode), lookup(mem, pos+1, b_mode));
    let dest = mem[pos+2] as usize;
    mem[dest] = ret;
    pos + 3
}

fn eval_store(mem: &mut Vec<i32>, pos : usize) -> usize {
    let mut buffer = String::new();
    println!("input: ");
    io::stdin().read_line(&mut buffer).unwrap();
    let val : i32 = buffer.trim_end().parse().unwrap();
    let dest = mem[pos] as usize;
    mem[dest] = val;
    pos + 1
}

fn eval_out(mem: &mut Vec<i32>, pos : usize) -> usize {
    let val = mem[mem[pos] as usize];
    println!("out: {}", val);
    pos + 1
}

// Evaluate the instruction at `pos`, and return the position of the next
// instruction
fn eval_at(mem : &mut Vec<i32>, pos : usize) -> usize {
    let full_op = mem[pos];
    let op = full_op % 100;
    let a_mode = (full_op / 100 % 2) > 0;
    let b_mode = (full_op / 1000 % 2) > 0;
    let c_mode = (full_op / 10000 % 2) > 0;
    match op {
        1 => eval_bin(mem, pos+1, |x,y| x + y, a_mode, b_mode),
        2 => eval_bin(mem, pos+1, |x,y| x * y, a_mode, b_mode),
        3 => eval_store(mem, pos+1),
        4 => eval_out(mem, pos + 1),
        99 => mem.len(),
        _ => panic!("unknown opcode {}", mem[pos]),
    }
}

fn read_prog(reader : &mut BufReader<File>) -> Vec<i32> {
    let mut mem : Vec<i32> = Vec::new();
    for line in reader.lines() {
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
    let args : Vec<String> = env::args().collect();
    let program = &args[1];
    let file = File::open(program).unwrap();
    let mut reader = BufReader::new(file);
    let mut prog = read_prog(&mut reader);
    run_prog(&mut prog);
}
