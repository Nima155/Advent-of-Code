mod parsers;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

use parsers::{
    find_winning_combination, parse_input_to_grids, produce_rotations_and_flippations_d,
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

    let mut memo = HashMap::new();

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
                &mut memo,
                *g.0,
                i as u32,
            );
            if ans.0 {
                println!("{:?}", ans.2);
                println!(
                    "{:?}",
                    ans.2[0][0].0 as u128
                        * ans.2[0].last().unwrap().0 as u128
                        * ans.2.last().unwrap().last().unwrap().0 as u128
                        * ans.2.last().unwrap().iter().next().unwrap().0 as u128
                );
                break 'a;
            }
        }
    }
}
