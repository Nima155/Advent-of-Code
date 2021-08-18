use std::{collections::HashSet, fs};

fn main() {
    match fs::read_to_string("input.txt") {
        Ok(z) => {
            let one = z
                .split("\r\n")
                .map(|c| i32::from_str_radix(c, 10).unwrap())
                .collect::<HashSet<_>>();
            
            for i in &one {
                for j in &one {
                    if one.contains(&(2020 - (i + j))) {
                        println!("{} {} {} {}", i, j,  2020 - (i + j), i * j * (2020 - (i + j)));
                    }
                }
            }
            
        }
        Err(error) => {
            eprintln!("{:?}", error)
        }
    }
}