use std::collections::{HashMap, HashSet};

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

fn flip(vec: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    vec.iter().rev().map(|e| e.clone()).collect()
    // .map(|r| r.iter().rev().map(|c| *c).collect::<Vec<_>>())
    // .collect()
}

fn rotate(vec: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    vec.iter()
        .enumerate()
        .map(|(i, _row)| vec.iter().map(|f| f[i]).rev().collect::<Vec<_>>())
        .collect()
}

pub fn produce_rotations_and_flippations_d(vec: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut ans = Vec::with_capacity(8);

    ans.push(vec.clone());

    for _i in 0..4 {
        let last = ans.last().unwrap();
        let rotated = rotate(last);

        let flipped = flip(&rotated);

        ans.push(flipped);
        ans.push(rotated);
    }

    return ans;
}

fn compare_borders(
    parent: &Vec<Vec<char>>,
    child: &Vec<Vec<char>>,
    parent_is_on_top: bool,
) -> bool {
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
    (threshold, mut row): (i32, i32),
    combinations: &HashMap<u32, Vec<Vec<Vec<char>>>>,
    cache: &mut HashMap<(u32, usize, i32, u32, u32, u32), bool>, // id row col idT idL orientation
    last: u32,
    last_orientation: u32,
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
                    let row_length = if map_t_len != 0 { map_t_len - 1 } else { 0 };
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
                        cache,
                        last,
                        last_orientation,
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
