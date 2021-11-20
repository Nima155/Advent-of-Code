#![feature(string_remove_matches)]
use std::fs;
fn main() {
    let mut line = fs::read_to_string("./input.txt").unwrap();
    line.remove_matches(" ");
    let line = line.chars().collect::<Vec<_>>();

    println!("{:?}", recursive_digger(&line));
}
// take .. repeat
fn recursive_digger(line: &[char]) -> i64 {
    let (mut i, mut virtual_indx, mut iswithin) = (0, 0_i64, false);

    while i < line.len() {
        if line[i].is_digit(10) {
            let (mut j, mut take, mut rep, mut is_on_rep) = (i, 0_i64, 0_i64, false);
            while line[j] != ')' {
                if line[j] == 'x' {
                    is_on_rep = true;
                } else if is_on_rep {
                    rep *= 10;
                    rep += line[j].to_digit(10).unwrap() as i64;
                } else {
                    take *= 10;
                    take += line[j].to_digit(10).unwrap() as i64;
                }
                j += 1;
            }
            iswithin = false;
            virtual_indx += rep * recursive_digger(&line[j + 1..j + 1 + take as usize]);
            i = j + 1 + take as usize;
            continue;
        } else if line[i] == '(' {
            iswithin = true;
        }
        if !iswithin {
            virtual_indx += 1;
        }
        i += 1;
    }
    virtual_indx
}
