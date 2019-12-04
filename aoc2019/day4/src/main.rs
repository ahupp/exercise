fn is_valid(ab : &[u8]) -> bool {

    let mut has_dub = false;
    let mut has_asc = true;

    // All are ascending
    for j in 1..ab.len() {
        if ab[j] < ab[j-1] {
            has_asc = false;
        }
    }

    // Find runs of exactly 2
    let mut idx = 0;
    while idx < ab.len() {
        let target = ab[idx];
        let mut found = 0;
        while idx < ab.len() && ab[idx] == target {
            idx += 1;
            found += 1;
        }
        if found == 2 {
            has_dub = true;
        }
    }
    has_dub && has_asc
}

fn countit(start : i32, end : i32) -> i32 {
    let mut nvalid = 0;
    for i in start..(end+1) {
        let s = format!("{}", i);
        if is_valid(s.as_bytes()) {
            nvalid += 1;
        }
    }
    nvalid
}

fn main() {
    println!("{}", countit(134792, 675810));
}
