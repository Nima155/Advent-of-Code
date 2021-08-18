use std::{collections::HashSet, fs};
fn main() {
    match fs::read_to_string("input.txt") {
        Ok(z) => {
            let one = z
                .split("\r\n")
                .map(|c| i32::from_str_radix(c, 10).unwrap())
                .collect::<HashSet<_>>();
            let ans = one.iter().find(|f| one.contains(&(2020 - *f))).unwrap();
            println!("{} {} {}", ans, 2020 - ans, ans * (2020 - ans));
            
        }
        Err(error) => {
            eprintln!("{:?}", error)
        }
    }
}
