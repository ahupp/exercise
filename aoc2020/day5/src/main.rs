use std::io::{self, BufRead};
use std::cmp;

fn find_seat(ch: &[char], size: u32) -> u32 {
    let mut start = 0;
    let mut end = size;

    for i in ch {
        let jump = (end - start) / 2;
        let i = *i;
        if i == 'F' || i == 'L' {
            end = end - jump;
        } else if i == 'B' || i == 'R' {
            start = start + jump;
        } else {
            panic!("unexpected value {}", i);
        }
    }

    return start;
}

fn main() {
    let mut seats = Vec::new();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        let ch : Vec<char> = i.chars().collect();
        if ch.len() != 10 {
            panic!("unexpected len");
        }
        let row = find_seat(&ch[0..7],128);
        let col = find_seat(&ch[7..10], 8);
        let id = row * 8 + col;
        seats.push(id);
    }
    seats.sort();
    for i in 1..seats.len() {
        if seats[i] - seats[i - 1] == 2 {
            println!("{}", seats[i] - 1);
        }
    }
}