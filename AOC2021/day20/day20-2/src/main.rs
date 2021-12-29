use std::{collections::HashMap, fs};
fn main() {
    let data = fs::read_to_string("../input.txt").unwrap();

    let mut data = data.split("\r\n\r\n");

    let algo_string = data.next().unwrap().chars().collect::<Vec<_>>();

    let mut input_grid = data
        .next()
        .unwrap()
        .split("\r\n")
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|j| (((i + 75) as i64, (j.0 + 75) as i64), j.1))
                .collect::<HashMap<_, _>>()
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    let [mut y_lim_l, mut y_lim_u] = [-100, 300];

    for _ in 0..50 {
        expand(&mut input_grid, &algo_string, (y_lim_l, y_lim_u));
        y_lim_l -= 1;
        y_lim_u += 1;
    }

    println!(
        "{}",
        input_grid
            .iter()
            .filter(|((i, j), _)| *i >= 25 && *i <= 224 && *j >= 25 && *j <= 224)
            .filter(|(_, c)| **c == '#')
            .count()
    );
    // println!("{}", );
}

fn expand(
    input: &mut HashMap<(i64, i64), char>,
    algorithm: &[char],
    (y_range_l, y_range_u): (i64, i64),
) {
    let mut input_clone = input.clone();

    for y in y_range_l..=y_range_u {
        for x in y_range_l..=y_range_u {
            let mut shift = 0;
            let mut indx_in_algo = 0;
            for indx_1 in (-1..2).rev() {
                for indx_2 in (-1..2).rev() {
                    let is_hash = *input
                        .get(&(y as i64 + indx_1, x as i64 + indx_2))
                        .unwrap_or(&'.')
                        == '#';
                    indx_in_algo |= (is_hash as i64) << shift;
                    shift += 1;
                }
            }
            input_clone.insert((y, x), algorithm[indx_in_algo as usize]);
        }
    }

    *input = input_clone;
}
// 5771 too high 5534 too high 5426 not right! 5324 not right! 5422 5434
