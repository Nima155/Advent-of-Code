use std::{collections::HashSet, fs};

fn main() {
    match fs::read_to_string("input.txt") {
        Ok(z) => {
            let one = z
                .split("\r\n")
                .map(|c| i32::from_str_radix(c, 10).unwrap())
                .collect::<HashSet<_>>();

            'search: // even loops can have lifetimes
            for i in &one {
                for j in &one {
                    if one.contains(&(2020 - (i + j))) {
                        println!("{} {} {} {}", i, j,  2020 - (i + j), i * j * (2020 - (i + j)));
                        break 'search; // exit the outer loop and therefore this loop.. can also say break 'search sth
                    } // label/lifetime can also be used with continue...
                }
            }
            
        }
        Err(error) => {
            eprintln!("{:?}", error)
        }
    }
}