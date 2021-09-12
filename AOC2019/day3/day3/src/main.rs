mod algorithms;
use std::{collections::HashSet, fs};

use algorithms::{find_closest_to_point_of_origin, find_visited_points};
fn main() {
    // closest point of intersection to central
    let read = fs::read_to_string("../input.txt").unwrap();

    let instructions = read
        .split("\r\n")
        .map(|f| {
            f.split(',')
                .map(|f| (&f[0..1], *&f[1..].parse::<u64>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut the_two_sets_of_visited_points: [HashSet<(i32, i32)>; 2] = [HashSet::new(), HashSet::new()];

    for (i, instruction) in instructions.iter().enumerate() {
        the_two_sets_of_visited_points[i] = find_visited_points(&instruction);
    }

    let new_one = the_two_sets_of_visited_points[0].intersection(&the_two_sets_of_visited_points[1]).collect::<HashSet<_>>();

    println!("{}", find_closest_to_point_of_origin(&new_one));
}
