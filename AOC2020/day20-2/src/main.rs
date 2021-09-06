mod parsers;
use crate::parsers::{build_picture, find_nessy};

use parsers::{
    find_winning_combination, parse_input_to_grids, produce_rotations_and_flippations_d,
};

use std::{
    collections::{HashMap, HashSet},
    fs,
};
fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let grids = parse_input_to_grids(&read);

    let grids_in_all_positions = grids
        .iter()
        .map(|f| {
            (
                *f.0,
                produce_rotations_and_flippations_d(f.1)
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<u32, Vec<Vec<Vec<char>>>>>();

    'a: for g in grids.iter() {
        for (i, _) in grids_in_all_positions.get(g.0).unwrap().iter().enumerate() {
            let mut map_to_map = vec![];
            let mut vis = HashSet::new();
            vis.insert(*g.0);
            map_to_map.push(vec![(*g.0, i)]);

            let ans = find_winning_combination(
                &grids,
                map_to_map,
                vis,
                ((grids.len() as f32).sqrt() as i32, 0),
                &grids_in_all_positions,
            );
            if ans.0 {
                let pic = build_picture(&ans.2, &grids_in_all_positions);

                for transformed_pic in produce_rotations_and_flippations_d(&pic) {
                    let ans = find_nessy(&transformed_pic);
                    if ans.1 != 0 {
                        println!("{}", ans.0 - ans.1);
                        break 'a;
                    }
                }

                // break 'a;
            }
        }
    }
}
