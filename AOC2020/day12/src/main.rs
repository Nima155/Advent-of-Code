use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;

macro_rules! make_map {
    ($($k:expr => $val:expr),+) => {{
        let mut maps = HashMap::new();

        $(maps.insert($k, $val);)+

        maps
    }};
}
/*
    Action N means to move north by the given value.
    Action S means to move south by the given value.
    Action E means to move east by the given value.
    Action W means to move west by the given value.
    Action L means to turn left the given number of degrees.
    Action R means to turn right the given number of degrees.
    Action F means to move forward by the given value in the direction the ship is currently facing.
*/

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let map = make_map!('E' => ['N', 'W', 'S'], 'W' => ['S', 'E', 'N'], 'S' => ['E', 'N', 'W'], 'N' => ['W','S','E']);

    let (mut y, mut x) = (0, 0);

    let mut cur_dir = 'E';

    read.split("\r\n").for_each(|e| {
        let chars = e.chars().collect::<Vec<char>>();
        let number_of_moves: i32 = String::from_iter(&chars[1..]).parse::<i32>().unwrap();
        match chars[0] {
            'N' | 'S' | 'E' | 'W' => {
                y += if chars[0] == 'N' {
                    -number_of_moves
                } else {
                    number_of_moves * ((chars[0] == 'S') as i32)
                };
                x += if chars[0] == 'W' {
                    -number_of_moves
                } else {
                    number_of_moves * ((chars[0] == 'E') as i32)
                };
            }
            'L' | 'R' => {
                if number_of_moves != 360 {
                    let indx = (number_of_moves / 90 - 1) as usize;

                    let slice = map.get(&cur_dir).unwrap();

                    if chars[0] == 'R' {
                        cur_dir = *slice.iter().rev().nth(indx).unwrap();
                    } else {
                        cur_dir = *slice.get(indx as usize).unwrap();
                    }
                }
            }
            'F' => match cur_dir {
                'E' => x += number_of_moves,
                'W' => x -= number_of_moves,
                'S' => y += number_of_moves,
                'N' => y -= number_of_moves,
                _ => {}
            },
            _ => {}
        }
    });
    println!("{}", y.abs() + x.abs())
}
