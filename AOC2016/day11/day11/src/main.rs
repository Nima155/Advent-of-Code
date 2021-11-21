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
            let (before, after): (i64, i64) = (cur_floor as i64 - 1, cur_floor as i64 + 1);
            let mut floors_copy = floors;

            let value = (floors[cur_floor] & (1 << i))  != 0;
            let generator = (floors_copy[cur_floor] & (1 << (i + shift))) != 0;
            
            
            if value && after < 4 {
                floors_copy[cur_floor] ^= 1 << i;
                floors_copy[after as usize] |= 1 << i;
                if !visited.contains(&(floors_copy, after as usize)) {
                    queue.push_back((floors_copy, steps + 1, after as usize));
                }
                floors_copy = floors;
            }
            if value && before >= 0 {
                floors_copy[cur_floor] ^= 1 << i;
                floors_copy[before as usize] |= 1 << i;
                if !visited.contains(&(floors_copy, before as usize)) {
                    queue.push_back((floors_copy, steps + 1, before as usize));

                }
                floors_copy = floors;
            }
            if generator && after < 4 {
                floors_copy[cur_floor] ^= 1 << (i + shift);
                floors_copy[after as usize] |= 1 << (i + shift);
                if !visited.contains(&(floors_copy, after as usize))  {

                    queue.push_back((floors_copy, steps + 1, after as usize));
                }
                floors_copy = floors;
            }
            if generator && before >= 0 {
                floors_copy[cur_floor] ^= 1 << (i + shift);
                floors_copy[before as usize] |= 1 << (i + shift);
                if !visited.contains(&(floors_copy, before as usize)) {

                    queue.push_back((floors_copy, steps + 1, before as usize));
                }
                floors_copy = floors;
            }

            for j in 0..shift {
                if i != j  {
                    let (gen_2, val_2) = (
                        (floors_copy[cur_floor] & (1 << (j + shift))) != 0,
                        (floors_copy[cur_floor] & (1 << j)) != 0,
                    );
                    if gen_2 && value {
                        if after < 4 {
                            floors_copy[cur_floor] ^= (1 << i) | (1 << (j + shift));
                            floors_copy[after as usize] |= (1 << i) | (1 << (j + shift));
                            if !visited.contains(&(floors_copy, after as usize)) {

                                queue.push_back((floors_copy, steps + 1, after as usize));
                            }
                            floors_copy = floors;
                        }
                        if before >= 0 {
                            floors_copy[cur_floor] ^= (1 << i) | (1 << (j + shift));
                            floors_copy[before as usize] |= (1 << i) | (1 << (j + shift));
                            if !visited.contains(&(floors_copy, before as usize)) {

                                queue.push_back((floors_copy, steps + 1, before as usize));
                            }
                            floors_copy = floors;
                        }
                    }
                    if gen_2 && generator {
                        if after < 4 {
                            floors_copy[cur_floor] ^= (1 << (i + shift)) | (1 << (j + shift));
                            floors_copy[after as usize] |= (1 << (i + shift)) | (1 << (j + shift));
                            if !visited.contains(&(floors_copy, after as usize)) {

                                queue.push_back((floors_copy, steps + 1, after as usize));
                            }
                            floors_copy = floors;
                        }
                        if before >= 0 {
                            floors_copy[cur_floor] ^= (1 << (i + shift)) | (1 << (j + shift));
                            floors_copy[before as usize] |= (1 << (i + shift)) | (1 << (j + shift));
                            if !visited.contains(&(floors_copy, before as usize)) {

                                queue.push_back((floors_copy, steps + 1, before as usize));
                            }
                            floors_copy = floors;
                        }
                    }

                    if val_2 && value {
                        if after < 4 {
                            floors_copy[cur_floor] ^= (1 << (i)) | (1 << (j));
                            floors_copy[after as usize] |= (1 << (i)) | (1 << (j));
                            if !visited.contains(&(floors_copy, after as usize)) {

                                queue.push_back((floors_copy, steps + 1, after as usize));
                            }
                            floors_copy = floors;
                        }
                        if before >= 0 {
                            floors_copy[cur_floor] ^= (1 << (i)) | (1 << (j));
                            floors_copy[before as usize] |= (1 << (i)) | (1 << (j));
                            if !visited.contains(&(floors_copy, before as usize)) {

                                queue.push_back((floors_copy, steps + 1, before as usize));
                            }
                            floors_copy = floors;
                        }
                    }
                }
            }
        }
        
    }
    0
}
