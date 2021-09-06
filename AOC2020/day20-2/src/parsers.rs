use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_input_to_grids(read: &str) -> HashMap<u32, Vec<Vec<char>>> {
    let ret = read
        .split("\r\n\r\n")
        .map(|grid| {
            let (mut k, mut v) = (0, Vec::with_capacity(10));

            grid.split("\r\n").enumerate().for_each(|(i, row)| {
                if i == 0 {
                    let num = row.split_at(4).1;
                    k = num[1..num.len() - 1].parse().unwrap();
                } else {
                    v.push(row.chars().collect::<Vec<_>>())
                }
            });

            (k, v)
        })
        .collect();

    ret
}

fn flip(vec: &[Vec<char>]) -> Vec<Vec<char>> {
    vec.iter().rev().cloned().collect()
    // .map(|r| r.iter().rev().map(|c| *c).collect::<Vec<_>>())
    // .collect()
}

fn rotate(vec: &[Vec<char>]) -> Vec<Vec<char>> {
    vec.iter()
        .enumerate()
        .map(|(i, _row)| vec.iter().map(|f| f[i]).rev().collect::<Vec<_>>())
        .collect()
}

pub fn produce_rotations_and_flippations_d(vec: &[Vec<char>]) -> Vec<Vec<Vec<char>>> {
    let mut ans = Vec::with_capacity(8);

    ans.push(vec.to_owned());

    for _i in 0..4 {
        let last = ans.last().unwrap();
        let rotated = rotate(last);

        let flipped = flip(&rotated);

        ans.push(flipped);
        ans.push(rotated);
    }

    ans
}

fn compare_borders(parent: &[Vec<char>], child: &[Vec<char>], parent_is_on_top: bool) -> bool {
    if parent_is_on_top {
        // println!("{:?} {:?}", parent.last().unwrap(), child.first().unwrap());
        parent.last().unwrap() == child.first().unwrap()
    } else {
        parent
            .iter()
            .map(|row| row.last().unwrap())
            .eq(child.iter().map(|row| row.first().unwrap()))
    }
}

pub fn find_winning_combination(
    grids: &HashMap<u32, Vec<Vec<char>>>,
    mut map_so_far: Vec<Vec<(u32, usize)>>, // [col[row[grid[....]]]]
    mut vis: HashSet<u32>,
    (threshold, row): (i32, i32),
    combinations: &HashMap<u32, Vec<Vec<Vec<char>>>>,
) -> (bool, HashSet<u32>, Vec<Vec<(u32, usize)>>) {
    if vis.len() == grids.len() {
        return (true, vis, map_so_far);
    }

    for id in grids.keys() {
        if !vis.contains(id) {
            for (j, grid) in combinations.get(id).unwrap().iter().enumerate() {
                let (cur_row_len, map_t_len) = (map_so_far.last().unwrap().len(), map_so_far.len());

                if (map_t_len <= 1
                    || compare_borders(
                        &combinations[&map_so_far[map_t_len - 2][cur_row_len].0]
                            [map_so_far[map_t_len - 2][cur_row_len].1],
                        grid,
                        true,
                    ))
                    && (cur_row_len == 0
                        || compare_borders(
                            &combinations[&map_so_far[map_t_len - 1][cur_row_len - 1].0]
                                [map_so_far[map_t_len - 1][cur_row_len - 1].1],
                            grid,
                            false,
                        ))
                {
                    let (vis_bef, map_bef) = (vis.clone(), map_so_far.clone());
                    vis.insert(*id);

                    map_so_far.last_mut().unwrap().push((*id, j));

                    if cur_row_len + 1 == threshold as usize && vis.len() != grids.len() {
                        map_so_far.push(Vec::with_capacity(threshold as usize));
                    }

                    let (ans, vis_1, mapo) = find_winning_combination(
                        grids,
                        map_so_far,
                        vis,
                        (threshold, row),
                        combinations,
                    );
                    if ans {
                        return (ans, vis_1, mapo);
                    }

                    vis = vis_bef;
                    map_so_far = map_bef;
                }
            }
        }
    }

    // cache.insert(
    //     (last, cur_row_len, row_1, id_top, id_left, last_orientation),
    //     false,
    // );
    (false, vis, map_so_far)
}

pub fn find_nessy(map: &[Vec<char>]) -> (i32, i32) {
    let (mut ans, mut tots) = (0, 0);

    lazy_static! {
        static ref RE_FAT: Regex = Regex::new(r###"#.{4}#{2}.{4}#{2}.{4}#{3}"###).unwrap();
        static ref RE_SLIM: Regex = Regex::new(r###"#.{2}#.{2}#.{2}#.{2}#.{2}#"###).unwrap();
    };

    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' && i + 2 < map.len() && j + 1 < row.len() && j >= 18 {
                let fat_slice = String::from_iter(&map[i + 1][j - 18..j + 2]);
                let slim_slice = String::from_iter(&map[i + 2][j - 17..j - 1]);

                if RE_FAT.is_match(&fat_slice) && RE_SLIM.is_match(&slim_slice) {
                    ans += 15;
                }
            }
            tots += (*c == '#') as i32;
        }
    }

    (tots, ans)
}

pub fn build_picture(
    done: &[Vec<(u32, usize)>],
    combinations: &HashMap<u32, Vec<Vec<Vec<char>>>>,
) -> Vec<Vec<char>> {
    let mut pic_in_chars = vec![];

    for pic in done {
        let mut cur_row = vec![];
        for row in pic {
            cur_row.push(&combinations.get(&row.0).unwrap()[row.1]);
        }

        for i in 1..cur_row[0][0].len() - 1 {
            let mut entire_row = vec![];
            for j in 0..cur_row.len() {
                entire_row.extend(cur_row[j][i][1..cur_row[j][i].len() - 1].iter());
            }
            pic_in_chars.push(entire_row);
        }
    }
    pic_in_chars
}
