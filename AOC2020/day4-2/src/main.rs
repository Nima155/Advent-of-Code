mod validators;

use std::fs;

use validators::{
    eye_color_validator, four_digit_validator, hair_color_validator, height_validator,
    pid_validator,
};

// "byr" =>, (Birth Year)
// "iyr" =>, (Issue Year)
// "eyr" =>, (Expiration Year)
// "hgt" =>, (Height)
// "hcl" =>, (Hair Color)
// "ecl" =>, (Eye Color)
// "pid" =>, (Passport ID)
// "cid" =>, (Country ID) => fine if missing

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

fn is_valid(pass: Vec<&str>) -> Option<Vec<&str>> {
    let mut ans = 0;

    pass.iter().for_each(|e| {
        e.split(' ').for_each(|entry| {
            let key_val = entry.split(':').collect::<Vec<_>>();

            match key_val[0] {
                "byr" => ans += four_digit_validator(key_val[1], 1920, 2002),
                "iyr" => ans += four_digit_validator(key_val[1], 2010, 2020),
                "eyr" => ans += four_digit_validator(key_val[1], 2020, 2030),
                "hgt" => ans += height_validator(key_val[1]),
                "hcl" => ans += hair_color_validator(key_val[1]),
                "ecl" => ans += eye_color_validator(key_val[1]),
                "pid" => ans += pid_validator(key_val[1]),
                _ => {}
            }
        })
    });

    if ans == 7 {
        Some(pass)
    } else {
        None
    }
}

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    println!(
        "{}",
        read.split("\r\n\r\n")
            .filter_map(|e| is_valid(e.split("\r\n").collect::<Vec<_>>()))
            .count()
    );
}
