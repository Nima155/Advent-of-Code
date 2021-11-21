use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

const ELEMENTS: [&str; 5] = [
    "thulium",
    "plutonium",
    "promethium",
    "strontium",
    "ruthenium",
];
// const ELEMENTS_2: [&str; 2] = [
//     "hydrogen", "lithium"
// ];
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    lazy_static! {
        static ref RE: Regex = Regex::new(r###"\w+-?\w+\s(microchip|generator)"###).unwrap();
    }

    let vc = lines
        .split("\r\n")
        .map(|e| {
            let temp_v = RE
                .find_iter(e)
                .map(|matc| matc.as_str())
                .collect::<Vec<_>>();
            let mut bit_set = 0;
            for item in temp_v {
                if item.contains("generator") {
                    let ele = item.split(' ').next().unwrap();
                    bit_set |= 1 << (ELEMENTS.iter().position(|e| *e == ele).unwrap() + 5)
                } else {
                    let ele = item.split('-').next().unwrap();
                    bit_set |= 1 << ELEMENTS.iter().position(|e| *e == ele).unwrap()
                }
            }
            bit_set
        })
        .collect::<Vec<_>>();

    println!("{}", search_and_move([vc[0], vc[1], vc[2], vc[3]], 5));
    // GET ALL TO 4TH floor
}

fn is_valid_combination(floors: [i64; 4], shift: i64) -> bool {
    for i in 0..4 {
        for j in 0..shift {
            if (floors[i] & (1 << j)) != 0
                && (floors[i] & 0b1111100000) != 0
                && ((floors[i] & (1 << (j + shift))) == 0)
            {
                return false;
            }
        }
    }
    true
}

const FLOORS: i64 = 4;
fn fill_and_remove(
    floors: &mut [i64; 4],
    cur_floor: usize,
    mask: i64,
    visited: &mut HashSet<([i64; 4], usize)>,
    queue: &mut VecDeque<([i64; 4], i64, usize)>,
    steps: i64,
    floors_before: &[i64; 4],
) {
    let floor_i64 = cur_floor as i64;
    let (bef, after) = (floor_i64 - 1, floor_i64 + 1);

    if bef >= 0 {
        floors[cur_floor] ^= mask;
        floors[bef as usize] |= mask;
        if !visited.contains(&(*floors, bef as usize)) {
            queue.push_back((*floors, steps + 1, bef as usize));
        }
        *floors = *floors_before;
    }
    if after < FLOORS {
        floors[cur_floor] ^= mask;
        floors[after as usize] |= mask;
        if !visited.contains(&(*floors, after as usize)) {
            queue.push_back((*floors, steps + 1, after as usize));
        }
        *floors = *floors_before;
    }
}

fn search_and_move(floor_contents: [i64; 4], shift: i64) -> i64 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((floor_contents, 0, 0));

    while !queue.is_empty() {
        let (floors, steps, cur_floor) = queue.pop_front().unwrap();

        if !is_valid_combination(floors, shift) || visited.contains(&(floors, cur_floor)) {
            continue;
        }

        if floors[3] == 0b1111111111 {
            return steps;
        }

        visited.insert((floors, cur_floor));
        for i in 0..shift {
            let mut floors_copy = floors;
            let value = (floors[cur_floor] & (1 << i)) != 0;
            let generator = (floors_copy[cur_floor] & (1 << (i + shift))) != 0;

            if value {
                fill_and_remove(
                    &mut floors_copy,
                    cur_floor,
                    1 << i,
                    &mut visited,
                    &mut queue,
                    steps,
                    &floors,
                );
            }
            if generator {
                fill_and_remove(
                    &mut floors_copy,
                    cur_floor,
                    1 << (i + shift),
                    &mut visited,
                    &mut queue,
                    steps,
                    &floors,
                );
            }

            for j in 0..shift {
                if i != j {
                    let (gen_2, val_2) = (
                        (floors_copy[cur_floor] & (1 << (j + shift))) != 0,
                        (floors_copy[cur_floor] & (1 << j)) != 0,
                    );
                    if gen_2 && value {
                        fill_and_remove(
                            &mut floors_copy,
                            cur_floor,
                            (1 << i) | (1 << (j + shift)),
                            &mut visited,
                            &mut queue,
                            steps,
                            &floors,
                        );
                    }
                    if gen_2 && generator {
                        fill_and_remove(
                            &mut floors_copy,
                            cur_floor,
                            (1 << (i + shift)) | (1 << (j + shift)),
                            &mut visited,
                            &mut queue,
                            steps,
                            &floors,
                        );
                    }

                    if val_2 && value {
                        fill_and_remove(
                            &mut floors_copy,
                            cur_floor,
                            (1 << (i)) | (1 << (j)),
                            &mut visited,
                            &mut queue,
                            steps,
                            &floors,
                        );
                    }
                }
            }
        }
    }
    0
}
