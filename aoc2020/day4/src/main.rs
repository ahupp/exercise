
use std::io::{self, BufRead};
use std::vec::Vec;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let pat = Regex::new(r"(?:(\w+):([^\s]+))").unwrap();
    let mut passport = HashMap::new();
    let mut all = Vec::new();
    for i in io::stdin().lock().lines() {
        let i = i.unwrap();
        if i.trim().len() == 0 {
            all.push(passport);
            passport = HashMap::new();
        }
        for c in pat.captures_iter(&i) {
            let key = &c[1];
            let value = &c[2];
            passport.insert(key.to_owned(), value.to_owned());
        }
    }
    if passport.len() > 0 {
        all.push(passport);
    }

    let make_numval = |begin: i32, end: i32| {
        return move |field: &str| {
            if field.chars().next().unwrap() == '0' {
                return false;
            }
            let num : i32 = field.parse().unwrap();
            return num >= begin && num <= end;
        };
    };

    let re_val = |pat: &str| {
        let pat = Regex::new(pat).unwrap();
        return move |field: &str| {
            return pat.is_match(field);
        };
    };


    let hgt_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let inch_val = make_numval(59, 76);
    let cm_val = make_numval(150, 193);

    let hgt_val = |field: &str| {
        let mat = hgt_re.captures(field);
        match mat {
            None => false,
            Some(cap) => {
                let unit = &cap[2];
                let val = &cap[1];
                if unit == "in" {
                    return inch_val(val);
                } else if unit == "cm" {
                    return cm_val(val);
                } else {
                    return false;
                }
            }
        }
    };

    let byr_val = make_numval(1920, 2002);
    let iyr_val = make_numval(2010, 2020);
    let eyr_val = make_numval(2020, 2030);
    let hcl_val = re_val(r"^#[0-9a-f]{6}$");
    let ecl_val = re_val(r"^(amb|blu|brn|gry|grn|hzl|oth)$");
    let pid_val = re_val(r"^\d{9}$");
    let req: [(&str, Box<dyn Fn(&str)->bool>); 7] = [
        ("byr", Box::new(&byr_val)),
        ("iyr", Box::new(&iyr_val)),
        ("eyr", Box::new(&eyr_val)),
        ("hgt", Box::new(&hgt_val)),
        ("hcl", Box::new(&hcl_val)),
        ("ecl", Box::new(&ecl_val)),
        ("pid", Box::new(&pid_val)),
        ];
    let is_valid = |pass: &HashMap<String,String>| {
        for (k, valid) in &req {
            if let Some(value) = pass.get(*k) {
                if !valid(value) {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
    };

    println!("{}", all.len());
    all.retain(is_valid);
    println!("{}", all.len());
}
