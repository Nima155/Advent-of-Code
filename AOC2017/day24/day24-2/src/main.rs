use std::{collections::HashSet, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let nums = lines
        .split("\r\n")
        .map(|l| {
            l.split('/')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (mut ans, mut length) = (0, 0);

    for pair in &nums {
        if pair[0] == 0 || pair[1] == 0 {
            let (length_1, strength) = find_maximum_strength(pair, 0, &mut HashSet::new(), &nums);
            if length_1 >= length {
                length = length_1;
                ans = i64::max(ans, strength);
            }
        }
    }
    println!("{}", ans);
}

fn find_maximum_strength(
    cur_node: &[i64],
    used_end: i64,
    visited: &mut HashSet<(i64, i64)>,
    combos: &[Vec<i64>],
) -> (i64, i64) {
    if visited.contains(&(cur_node[0], cur_node[1])) {
        return (0, 0);
    }

    visited.insert((cur_node[0], cur_node[1]));

    let mut intermediate_answers = vec![];
    let connecting_end = if used_end != cur_node[0] {
        cur_node[0]
    } else {
        cur_node[1]
    };

    for pair in combos {
        if pair[0] == connecting_end || pair[1] == connecting_end {
            intermediate_answers.push(find_maximum_strength(
                pair,
                connecting_end,
                &mut visited.clone(),
                combos,
            ));
        }
    }
    let chosen_elem = intermediate_answers.iter().max().unwrap();
    (1 + chosen_elem.0, used_end + connecting_end + chosen_elem.1)
}
