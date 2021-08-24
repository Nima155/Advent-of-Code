mod validations;
use std::fs;

use validations::{mutate_with_marked, validate_empty_seat, validate_occupied_seat};

use crate::validations::count_occupied;


fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let mut grid = read.split("\r\n").map(|e| e.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    
    
    loop {
        let mut marked = Vec::with_capacity(256);
        for (i,vec) in grid.iter().enumerate() {
            for (j, c) in vec.iter().enumerate() {
                    match *c {
                        'L' => {validate_empty_seat(&grid, i, j, &mut marked);}
                        '#' => {validate_occupied_seat(&grid, i, j, &mut marked);}
                        _ => {}
                    };
            }
        }
        let mutated_array = mutate_with_marked(grid.clone(), &marked);
        if mutated_array == grid {
            println!("{}", count_occupied(&grid));
            break
        }
        grid = mutated_array
    }
}
