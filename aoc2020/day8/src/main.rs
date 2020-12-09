use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};


#[derive(Copy, Clone)]
enum Instr {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn parse_inst(line: &str) -> Instr {
    let mut splat = line.split_whitespace();
    let instr = splat.next().unwrap();
    let arg = splat.next().unwrap();
    let argn : i64 = arg.parse().unwrap();

    match instr {
        "acc" => Instr::Acc(argn),
        "jmp" => Instr::Jmp(argn),
        "nop" => Instr::Nop(argn),
        _ => panic!("unexpected instr {}", line),
    }
}

fn run_prog(prog: &Vec<Instr>) -> Option<i64> {
    let mut acc: i64 = 0;
    let mut pc = 0;

    let mut visited = Vec::new();
    visited.resize(prog.len(), false);

    while pc < prog.len() {
        if visited[pc] {
            return None;
        }
        visited[pc] = true;
        match prog[pc] {
            Instr::Nop(_) => pc += 1,
            Instr::Acc(value) => {
                acc += value;
                pc += 1;
            },
            Instr::Jmp(value) => {
                pc = ((pc as i64) + value) as usize;
            }
        }
    }

    return Some(acc);
}

fn main() {
    let mut prog = Vec::new();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        prog.push(parse_inst(&i));
    }

    for i in 0..prog.len() {
        let orig = prog[i];
        prog[i] = match prog[i] {
            Instr::Nop(value) => Instr::Jmp(value),
            Instr::Jmp(value) => Instr::Nop(value),
            other => other,
        };
        match run_prog(&prog) {
            None => {},
            Some(acc) => println!("{}", acc),
        }
        prog[i] = orig;
    }
    run_prog(&prog);
}
