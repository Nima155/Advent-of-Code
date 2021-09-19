mod board;
use crate::board::find_all_the_visibles;
use board::vaporize;
use rayon::prelude::*;
use std::fs;

fn main() {
    let board = fs::read_to_string("../input.txt").unwrap();

    let mut board_to_vec = board
        .split("\r\n")
        .map(|f| f.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let points = board_to_vec
        .par_iter()
        .enumerate()
        .map(|(i, row)| {
            row.par_iter()
                .enumerate()
                .map(|(j, c)| {
                    if *c == '#' {
                        ((i, j), find_all_the_visibles(i, j, &board_to_vec).0)
                    } else {
                        ((i, j), 0)
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let point = points.iter().fold(((0_usize, 0_usize), 0), |acc, cur| {
        if cur.1 > acc.1 {
            *cur
        } else {
            acc
        }
    });
    // println!("{:?}", point);
    println!("{:?}", vaporize(point.0 .0, point.0 .1, &mut board_to_vec));
}
