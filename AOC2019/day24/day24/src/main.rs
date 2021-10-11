mod grid_ops;
use std::fs;

use grid_ops::conway_small;
fn main() {
    let grid_in_str = fs::read_to_string("../input.txt").unwrap();

    let starting_grid = grid_in_str
        .split("\r\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    conway_small(&starting_grid);
}
