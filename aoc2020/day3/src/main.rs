use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Eq, PartialEq)]
enum Square {
    Tree,
    Open,
}

fn tree_hit(board: &Vec<Vec<Square>>, slope_x: usize, slope_y: usize) -> usize {
    let mut cx = 0;
    let mut cy = 0;
    let mut ct = 0;
    while cy < board.len() {
        if board[cy][cx] == Square::Tree {
            ct += 1
        }
        cy += slope_y;
        cx += slope_x;
        cx = cx % board[0].len();
    }
    return ct;
}

fn main() {
    let mut board = Vec::new();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        let mut row = Vec::new();
        for c in i.chars() {
            if c == '.' {
                row.push(Square::Open);
            } else if c == '#' {
                row.push(Square::Tree);
            }
        }
        board.push(row);
    }
    let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut prod = 1;
    for (x,y) in &slopes {
        let ct = tree_hit(&board, *x, *y);
        prod *= ct;
    }

    println!("{}", prod);
}

