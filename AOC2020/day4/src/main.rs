use std::fs;

// "byr" =>, (Birth Year)
// "iyr" =>, (Issue Year)
// "eyr" =>, (Expiration Year)
// "hgt" =>, (Height)
// "hcl" =>, (Hair Color)
// "ecl" =>, (Eye Color)
// "pid" =>, (Passport ID)
// "cid" =>, (Country ID) => fine if missing

fn is_valid(pass: Vec<&str>) -> Option<Vec<&str>> {
    let mut ans = 0;
    let mut cid_is_not_there = true;

    pass.iter().for_each(|e| {
        e.split(' ').for_each(|entry| {
            let key = entry.split(':').collect::<Vec<_>>()[0];
            if key == "cid" {
                cid_is_not_there = false
            }
            match key {
                "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" | "cid" => ans += 1,
                _ => {}
            }
        })
    });

    if ans + (cid_is_not_there as u32) >= 8 {
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
