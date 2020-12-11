use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Seat {
    Empty,
    Occupied,
    Floor
}

fn occupied(room: &Vec<Vec<Seat>>, x: i32, y: i32) -> bool {
    if y < 0 || x < 0 {
        return false;
    }
    let x = x as usize;
    let y = y as usize;

    if y == room.len() {
        return false;
    }
    if x == room[y].len() {
        return false;
    }
    return room[y][x] == Seat::Occupied;
}

fn find_in_dir(room: &Vec<Vec<Seat>>, x: i32, y: i32, dir: (i32,i32)) -> Seat {
    let mut xi = x;
    let mut yi = y;
    let (dx, dy) = dir;
    loop {
        xi += dx;
        yi += dy;
        if xi < 0 || yi < 0 {
            return Seat::Empty;
        }
        let yi = yi as usize;
        let xi = xi as usize;
        if yi == room.len() || xi == room[0].len() {
            return Seat::Empty;
        }
        let seat = room[yi][xi];
        if seat == Seat::Floor {
            continue;
        } else {
            return seat;
        }
    }
}

fn num_occupied(room: &Vec<Vec<Seat>>, x: i32, y: i32) -> u32 {
    let mut ct = 0;

    for xd in &[-1, 0, 1] {
        for yd in &[-1, 0, 1] {
            if *xd == 0 && *yd == 0 {
                continue;
            }
            if find_in_dir(room, x, y, (*xd, *yd)) == Seat::Occupied {
                ct += 1;
            }
        }
    }
    return ct;
}

fn step(room: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    let mut next = Vec::new();
    for y in 0..room.len() {
        let mut nr = Vec::new();
        for x in 0..room[0].len() {
            let n = match room[y][x] {
                Seat::Floor => Seat::Floor,
                Seat::Empty => {
                    if num_occupied(room, x as i32, y as i32) == 0 {
                        Seat::Occupied
                    } else {
                        Seat::Empty
                    }
                },
                Seat::Occupied => {
                    if num_occupied(room, x as i32, y as i32) >= 5 {
                        Seat::Empty
                    } else {
                        Seat::Occupied
                    }
                }
            };
            nr.push(n);
        }
        next.push(nr);
    }

    return next;
}

fn print_room(room: &Vec<Vec<Seat>>) {
    println!("");
    for row in room {
        let ch : String = row.iter().map(|e| match e {
            Seat::Occupied => '#',
            Seat::Empty => 'L',
            Seat::Floor => '.',
        }).collect();
        println!("{:?}", ch);
    }
}

fn main() {
    let mut layout = Vec::new();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        let mut row = Vec::new();
        for c in i.chars() {
            let sym = match c {
                '#' => Seat::Occupied,
                'L' => Seat::Empty,
                '.' => Seat::Floor,
                _ => panic!("unknown value {}", c),
            };
            row.push(sym);
        }
        layout.push(row);
    }

    loop {
        print_room(&layout);
        let next = step(&layout);
        if next == layout {
            let mut ct = 0;
            for row in &layout {
                for seat in row {
                    if *seat == Seat::Occupied {
                        ct += 1;
                    }
                }
            }
            println!("{}", ct);
            break;
        }
        layout = next;
    }
}
