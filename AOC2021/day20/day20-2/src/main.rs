use std::{
    collections::{HashMap, HashSet},
    fs,
};
fn main() {
    let data = fs::read_to_string("../input.txt").unwrap();

    let mut data = data.split("\r\n\r\n");

    let algo_string = data.next().unwrap().chars().collect::<Vec<_>>();

    let input_grid = data
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
        // 515091 too high 16223 too high!
    let mut big_input_grid = vec![vec!['.'; input_grid.len() + 500]; input_grid.len() + 500];

    for ((y, x), v) in &input_grid {
        big_input_grid[*y as usize][*x as usize] = *v;
    }

    for i in 0..50 {
        expand(&mut big_input_grid, &algo_string);
    }

    println!(
        "{}",
        big_input_grid
            .iter().enumerate().filter(|(i, r)| *i >= 25 && *i <= (75 + 100 + 50))
            .map(|(i, row)| {
                row.iter().enumerate()
                    .filter(|(j, c)| *j >= 25 && *j <= (75 + 100 + 50) && **c == '#')
            })
            .flatten()
            .count()
    );

    // println!("{}", );
}

fn expand(big_input_grid: &mut Vec<Vec<char>>, algorithm: &[char]) {
    let mut clone = big_input_grid.to_owned();

    for (i, r) in big_input_grid.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            let mut shift = 0;
            let mut indx_in_algo = 0;
            if i != 0 && j != 0 && i + 1 < big_input_grid.len() && j + 1 < big_input_grid.len()  {
                for indx_1 in (-1..2).rev() {
                    for indx_2 in (-1..2).rev() {
                        {
                            let is_hash = big_input_grid[(i as i64 + indx_1) as usize]
                                [(j as i64 + indx_2) as usize]
                                == '#';
                            indx_in_algo |= (is_hash as i64) << shift;
                            shift += 1;
                        }
                    }
                }
                clone[i][j] = algorithm[indx_in_algo as usize];
            } 
        }
    }
    *big_input_grid = clone;
}
// 5771 too high 5534 too high 5426 not right! 5324 not right! 5422 5434
