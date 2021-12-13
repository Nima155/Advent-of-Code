// maxof - 10 % 6
// 741 is too high
// 636 is too low
use std::{collections::HashSet, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let mut lines_split = lines.split("\r\n\r\n");

    let mut occupied = lines_split
        .next()
        .unwrap()
        .split("\r\n")
        .map(|l| {
            let vc = l
                .split(',')
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (vc[0], vc[1])
        })
        .collect::<HashSet<_>>();

    let folds = lines_split
        .next()
        .unwrap()
        .split("\r\n")
        .map(|s| {
            let lst = s.split(' ').last().unwrap();
            let (by, am) = lst.split_at(1);

            (by, am[1..].parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    process_fold(&mut occupied, folds[0]);
    println!("{}", occupied.len());
}

fn process_fold(occupied: &mut HashSet<(i64, i64)>, fold: (&str, i64)) {
    let mut new_one = HashSet::new();
    for (x, y) in occupied.iter() {
        match fold.0 {
            "x" if *x > fold.1 => {
                new_one.insert(((fold.1 - 1) - (*x % (fold.1 + 1)), *y));
            }
            "y" if *y > fold.1 => {
                new_one.insert((*x, (fold.1 - 1) - *y % (fold.1 + 1)));
            }
            _ => {
                new_one.insert((*x, *y));
            }
        }
    }
    *occupied = new_one;
}
