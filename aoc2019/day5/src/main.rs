use std::io::{BufRead, self, BufReader};
use std::env;
use std::fs::File;

struct Prog {
    mem: Vec<i32>,
    ip: usize,
    modes: (bool, bool, bool),
    op: i32,
}

impl Prog {
    fn new(mem : Vec<i32>) -> Prog {
        let mut p = Prog {mem: mem, ip: 0, modes: (false, false, false), op: 0};
        p.set_ip(0);
        p
    }

    fn set_ip(&mut self, pos: usize) {
        self.ip = pos;
        let full_op = self.mem[self.ip];
        self.op = full_op % 100;
        let a_mode = ((full_op / 100) % 2) > 0;
        let b_mode = ((full_op / 1000) % 2) > 0;
        let c_mode = ((full_op / 10000) % 2) > 0;
        self.modes = (a_mode, b_mode, c_mode);
    }

    fn write(&mut self, idx: usize, value: i32) {
        let dest = self.mem[self.ip + idx] as usize;
        //println!("write {} {} {}", idx, dest, value);
        self.mem[dest] = value;
    }

    fn param(&self, i: usize) -> i32 {
        let pos = self.ip + i;
        let is_immediate = match i {
            1 => self.modes.0,
            2 => self.modes.1,
            3 => self.modes.2,
            _ => panic!("bad param index {}", i),
        };
        if is_immediate {
            self.mem[pos]
        } else {
            self.mem[self.mem[pos] as usize]
        }
        //println!(" param {} {} {}", i, a, is_immediate);
    }

    fn eval(&mut self) -> usize {
        //println!("op {}", self.op);
        match self.op {
            1 => self.eval_bin(|x,y| x + y),
            2 => self.eval_bin(|x,y| x * y),
            3 => self.eval_store(),
            4 => self.eval_out(),
            5 => self.eval_jump_if_true(),
            6 => self.eval_jump_if_false(),
            7 => self.eval_lt(),
            8 => self.eval_eq(),
            99 => self.mem.len(),
            _ => panic!("unknown opcode {}", self.op),
        }
    
    }

    fn run(&mut self) {
        loop {
            let ip = self.eval();
            if ip == self.mem.len() {
                break;
            }
            self.set_ip(ip);
        }
    }
    
    fn eval_bin<F : Fn(i32,i32)->i32>(&mut self, fun: F) -> usize {
        let ret = fun(self.param(1), self.param(2));
        self.write(3, ret);
        self.ip + 4
    }
    
    fn eval_store(&mut self) -> usize {
        let mut buffer = String::new();
        println!("input: ");
        io::stdin().read_line(&mut buffer).unwrap();
        let val : i32 = buffer.trim_end().parse().unwrap();
        self.write(1, val);
        self.ip + 2
    }
    
    fn eval_out(&mut self) -> usize {
        let val = self.param(1);
        println!("out: {}", val);
        self.ip + 2
    }    

    fn eval_jump_if_true(&mut self) -> usize {
        let v = self.param(1);
        if v > 0 {
            self.param(2) as usize
        } else {
            self.ip + 3
        }
    }

    fn eval_jump_if_false(&mut self) -> usize {
        let v = self.param(1);
        if v == 0 {
            self.param(2) as usize
        } else {
            self.ip + 3
        }
    }

    fn eval_lt(&mut self) -> usize {
        let a = self.param(1);
        let b = self.param(2);
        self.write(3, if a < b { 1 } else { 0 });
        self.ip + 4
    }

    fn eval_eq(&mut self) -> usize {
        let a = self.param(1);
        let b = self.param(2);
        self.write(3, if a == b { 1 } else { 0 });
        self.ip + 4
    }
}


fn read_prog(reader : &mut BufReader<File>) -> Prog {
    let mut mem : Vec<i32> = Vec::new();
    for line in reader.lines() {
        for entry in line.unwrap().split(",") {
            let as_int : i32 = entry.parse().unwrap();
            mem.push(as_int);
        }
    }
    Prog::new(mem)
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let program = &args[1];
    let file = File::open(program).unwrap();
    let mut reader = BufReader::new(file);
    let mut prog = read_prog(&mut reader);
    prog.run();
}
