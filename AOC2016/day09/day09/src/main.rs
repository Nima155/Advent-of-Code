#![feature(string_remove_matches)]
use std::fs;
fn main() {
    let mut line = fs::read_to_string("../input.txt").unwrap();
    line.remove_matches(" ");
    let line = line.chars().collect::<Vec<_>>();

    let (mut i, mut virtual_indx, mut iswithin) = (0, 0, false);

    while i < line.len() {
        if line[i].is_digit(10) {
            let (mut j, mut take, mut rep, mut is_on_rep) = (i, 0, 0, false);
            while line[j] != ')' {
                if line[j] == 'x' {
                    is_on_rep = true;
                }
                else if is_on_rep {
                    rep *= 10;
                    rep += line[j].to_digit(10).unwrap();
                } else {
                    take *= 10;
                    take += line[j].to_digit(10).unwrap();
                }
                j += 1; 
            }
            iswithin = false;
            virtual_indx += take * rep;
            i = j + 1 + take as usize;
            continue;
        }
        else if line[i] == '(' {
            iswithin = true;
        }
        if !iswithin {
            virtual_indx += 1;
        }
        i += 1;
    }
    println!("{}", virtual_indx);
}
// take .. repeat