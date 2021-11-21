use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

const ELEMENTS: [&str; 5] = ["thulium", "plutonium", "promethium", "strontium", "ruthenium"];

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
                    bit_set |= 1 << ELEMENTS.iter().position(|e| *e == ele).unwrap() + 5
                } else {
                    let ele = item.split('-').next().unwrap();
                    bit_set |= 1 << ELEMENTS.iter().position(|e| *e == ele).unwrap()
                }
            }
            bit_set
        })
        .collect::<Vec<_>>();
    
    println!(
        "{}",
        search_and_move(
            0,
            [vc[0], vc[1], vc[2], vc[3]],
            &mut HashMap::new(),
            &mut HashSet::new()
        )
    );
    // GET ALL TO 4TH floor
}

fn all_have_gens(floor_content: i64) -> bool {
    for i in 0..5 {
        if (floor_content & (1 << i)) != 0 && (floor_content & (1 << (i + 5))) == 0 {
            return false;
        }
    }
    true
}

fn search_and_move(
    cur_floor: i64,
    floor_contents: [i64; 4],
    cache: &mut HashMap<[i64; 4], i64>,
    visited: &mut HashSet<[i64; 4]>,
) -> i64 {
    
    if floor_contents[3] == 0b1111111111 {
        return 0;
    }
    if visited.contains(&floor_contents) {
        return 1000007;
    }
    if cache.contains_key(&floor_contents) {
        return *cache.get(&floor_contents).unwrap();
    }

    let mut floor_conts = floor_contents;
    let cur = floor_conts[cur_floor as usize];
    visited.insert(floor_contents);
    let mut ans = 100000007;
    for i in 0..5 {
        let (bef_fl, aft_fl) = (cur_floor - 1, cur_floor + 1);
        let (generator_1, value_1) = (cur & (1 << (i + 5)) != 0, cur & (1 << i) != 0);

        if value_1 && generator_1 {
            if aft_fl < 4 && all_have_gens(floor_conts[aft_fl as usize]) {
                floor_conts[cur_floor as usize] ^= (1 << i) | (1 << (i + 5));
                floor_conts[aft_fl as usize] |= (1 << i) | (1 << (i + 5));
                ans = i64::min(
                    ans,
                    1 + search_and_move(aft_fl, floor_conts, cache, visited),
                );
                floor_conts = floor_contents;
            }
            if bef_fl >= 0 && all_have_gens(floor_conts[bef_fl as usize]) {
                floor_conts[cur_floor as usize] ^= (1 << i) | (1 << (i + 5));
                floor_conts[bef_fl as usize] |= (1 << i) | (1 << (i + 5));
                ans = i64::min(
                    ans,
                    1 + search_and_move(bef_fl, floor_conts, cache, visited),
                );
                floor_conts = floor_contents;
            }
        }


        if value_1
            && aft_fl < 4
            && ((floor_conts[aft_fl as usize] >> 5) == 0
                || (floor_conts[aft_fl as usize] & 0b1111100000) >> (i + 5))
        {
            floor_conts[cur_floor as usize] ^= 1 << i;
            floor_conts[aft_fl as usize] |= 1 << i;
            ans = i64::min(
                ans,
                1 + search_and_move(aft_fl, floor_conts, cache, visited),
            );
            floor_conts = floor_contents;
        }
        if value_1
            && bef_fl >= 0
            && ((floor_conts[bef_fl as usize] >> 5) == 0
                || (floor_conts[bef_fl as usize] & 0b1111100000) == (1 << (i + 5)))
        {
            floor_conts[cur_floor as usize] ^= 1 << i;
            floor_conts[bef_fl as usize] |= 1 << i;
            ans = i64::min(
                ans,
                1 + search_and_move(bef_fl, floor_conts, cache, visited),
            );
            floor_conts = floor_contents;
        }


        if generator_1 && aft_fl < 4 && all_have_gens(floor_conts[aft_fl as usize]) {
            floor_conts[cur_floor as usize] ^= 1 << (i + 5);
            floor_conts[aft_fl as usize] |= 1 << (i + 5);
            ans = i64::min(
                ans,
                1 + search_and_move(aft_fl, floor_conts, cache, visited),
            );
            floor_conts = floor_contents;
        }
        if generator_1 && bef_fl >= 0 && all_have_gens(floor_conts[bef_fl as usize]) {
            floor_conts[cur_floor as usize] ^= 1 << (i + 5);
            floor_conts[bef_fl as usize] |= 1 << (i + 5);
            ans = i64::min(
                ans,
                1 + search_and_move(bef_fl, floor_conts, cache, visited),
            );
            floor_conts = floor_contents;
        }
        for j in 0..5 {
            let (generator_2, value_2) = (cur & (1 << (j + 5)) != 0, cur & (1 << j) != 0);
            if i != j {
                if generator_1
                    && generator_2
                    && !value_1
                    && !value_2
                    && aft_fl < 4
                    && (floor_contents[aft_fl as usize] == ((1 << i) | (1 << j))
                        || floor_contents[aft_fl as usize] & 0b0000011111 == 0)
                {
                    floor_conts[cur_floor as usize] ^= (1 << (i + 5)) | (1 << (j + 5));
                    floor_conts[aft_fl as usize] |= (1 << (i + 5)) | (1 << (j + 5));
                    ans = i64::min(
                        ans,
                        1 + search_and_move(aft_fl, floor_conts, cache, visited),
                    );
                    floor_conts = floor_contents;
                }

                if generator_1
                    && generator_2
                    && !value_1
                    && !value_2
                    && bef_fl >= 0
                    && (floor_contents[bef_fl as usize] == (1 << i | 1 << j)
                        || floor_contents[bef_fl as usize] & 0b0000011111 == 0)
                {
                    floor_conts[cur_floor as usize] ^= 1 << (i + 5) | 1 << (j + 5);
                    floor_conts[bef_fl as usize] |= 1 << (i + 5) | 1 << (j + 5);
                    ans = i64::min(
                        ans,
                        1 + search_and_move(bef_fl, floor_conts, cache, visited),
                    );
                    floor_conts = floor_contents;
                }
                if value_2
                    && value_1
                    && !generator_1
                    && !generator_2
                    && aft_fl < 4
                    && (floor_contents[aft_fl as usize] >> 5 == 0
                        || floor_contents[aft_fl as usize] & 0b1111100000
                            == ((1 << (i + 5)) | (1 << (j + 5))))
                {
                    floor_conts[cur_floor as usize] ^= (1 << i) | (1 << j);
                    floor_conts[aft_fl as usize] |= (1 << i) | (1 << j);
                    ans = i64::min(
                        ans,
                        1 + search_and_move(aft_fl, floor_conts, cache, visited),
                    );
                    floor_conts = floor_contents;
                }
                if value_2
                    && value_1
                    && !generator_1
                    && !generator_2
                    && bef_fl >= 0
                    && (floor_contents[bef_fl as usize] >> 5 == 0
                        || floor_contents[bef_fl as usize] & 0b1111100000
                            == ((1 << (i + 5)) | (1 << (j + 5))))
                {
                    floor_conts[cur_floor as usize] ^= (1 << i) | (1 << j);
                    floor_conts[bef_fl as usize] |= (1 << i) | (1 << j);
                    ans = i64::min(
                        ans,
                        1 + search_and_move(bef_fl, floor_conts, cache, visited),
                    );
                    floor_conts = floor_contents;
                }
            }
        }
    }
    cache.insert(floor_contents, ans);
    ans
}
