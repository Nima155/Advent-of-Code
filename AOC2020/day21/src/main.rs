mod finders;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

use finders::{find_the_actual_mappings, map_the_map, process_the_map};

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let lines_and_ings = read
        .split("\r\n")
        .enumerate()
        .map(|(i, line)| {
            let str = line.split('(').collect::<Vec<_>>();
            (
                i,
                (
                    str[0].split(' ').collect::<HashSet<_>>(),
                    str[1][..str[1].len() - 1]
                        .split(", ")
                        .map(|f| {
                            let mut iterator = f.split(' ');
                            if iterator.clone().count() == 1 {
                                f
                            } else {
                                iterator.nth(1).unwrap()
                            }
                        })
                        .collect::<HashSet<_>>(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    let (impo, _) = find_the_actual_mappings(process_the_map(map_the_map(&lines_and_ings)));
    let impo_vals = impo.values().collect::<HashSet<_>>();
    let mut anzi = 0;

    for v in lines_and_ings.values() {
        let mut ans = HashSet::new();
        for fd in &v.0 {
            if !fd.is_empty() && !impo_vals.contains(fd) {
                ans.insert(fd);
            }
        }

        anzi += ans.len();
    }
    let sorted = impo.iter().collect::<BTreeSet<_>>();

    println!("{}", anzi);

    println!(
        "{:?}",
        sorted.iter().map(|f| *f.1).collect::<Vec<_>>().join(",")
    );
}
