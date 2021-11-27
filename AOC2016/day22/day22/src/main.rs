use std::{collections::{HashMap, HashSet}, fs};
fn main() {
    let nodes = fs::read_to_string("../input.txt").unwrap();
    let nodes = nodes
        .split("\r\n")
        .skip(2)
        .map(|l| {
            let node_info = l.split_ascii_whitespace().collect::<Vec<_>>();
            let mut node_pos = node_info[0]
                .split('-')
                .skip(1)
                .map(|e| e[1..].parse::<i64>().unwrap());
            (
                (node_pos.next().unwrap(), node_pos.next().unwrap()),
                [
                    strip_and_parse(node_info[1], 'T'),
                    strip_and_parse(node_info[2], 'T'),
                    strip_and_parse(node_info[3], 'T'),
                    strip_and_parse(node_info[4], '%'),
                ],
            )
        })
        .collect::<HashMap<_, _>>();
    println!("{}", viable_nodes(&nodes))
}

fn strip_and_parse(num: &str, car_strip: char) -> i64 {
    num.strip_suffix(car_strip).unwrap().parse::<i64>().unwrap()
}

fn viable_nodes(nodes: &HashMap<(i64, i64), [i64; 4]>) -> i64 {
    let mut visited_comb = HashSet::new();
    
    for (k_1, [_size, used, _avail, _]) in nodes {
        for (k_2, [_size_1, _used_1, avail_1, _]) in nodes {
            if k_1 != k_2 && *used != 0 && *avail_1 >= *used {
                let mut comb = [k_1, k_2];
                comb.sort();
                visited_comb.insert(comb);
            }
        }
    }
    visited_comb.len() as i64
}