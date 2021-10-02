mod maze_walker;
use std::fs;

use maze_walker::{dijkstra, find_portals};
fn main() {
    let maze = fs::read_to_string("../input.txt").unwrap();

    let maze_vec = maze
        .split("\r\n")
        .map(|line| line.trim_end().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (all_portals_cept, end_point, start_point) = find_portals(&maze_vec);

    println!(
        "{}",
        dijkstra(&all_portals_cept, start_point, end_point, &maze_vec)
    );
}
