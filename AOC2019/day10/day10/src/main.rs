use rayon::prelude::*;
use std::fs;


fn main() {
    let board = fs::read_to_string("./mock-input.txt").unwrap();

    let board_to_vec = board
        .split("\r\n")
        .map(|f| f.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!(
        "{}",
        board_to_vec
            .par_iter()
            .enumerate()
            .map(|(i, row)| {
                row.par_iter()
                    .enumerate()
                    .map(|(j, c)| {
                        if *c == '#' {
                            find_all_the_visibles(i, j, &board_to_vec)
                        } else {
                            0
                        }
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    );
}

fn get_line((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> f64 {
    f64::atan2(y1 as f64 - y2 as f64, x1 as f64 - x2 as f64)
}

fn find_all_the_visibles(y: usize, x: usize, grid: &[Vec<char>]) -> usize {
    // type DoubleSet = (HashSet<f64>, HashSet<(usize, usize)>);

    let mut visited_gradients = Vec::new();

    for (y1, row) in grid.iter().enumerate() {
        for (x1, c) in row.iter().enumerate() {
            if *c == '#' && (y1, x1) != (y, x) {
                let point = get_line((x as i64, y as i64), (x1 as i64, y1 as i64));
                if !visited_gradients.contains(&point) {
                    visited_gradients.push(point);
                }
            }
        }
    }

    

    visited_gradients.len()
}
