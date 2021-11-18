use std::collections::HashMap;

fn flip(vec: &[Vec<char>]) -> Vec<Vec<char>> {
    vec.iter().rev().cloned().collect()
}

fn rotate(vec: &[Vec<char>]) -> Vec<Vec<char>> {
    vec.iter()
        .enumerate()
        .map(|(i, _row)| vec.iter().map(|f| f[i]).rev().collect::<Vec<_>>())
        .collect()
}

fn produce_rotations_and_flippations_d(vec: &[Vec<char>]) -> Vec<Vec<Vec<char>>> {
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

pub fn simulate_the_process(rules: &HashMap<&str, &str>, mut cycles: i64) {
    let mut initial_grid = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];
    let cycle_bef = cycles;
    while cycles > 0 {
        initial_grid = divide_and_split(&initial_grid, rules);
        cycles -= 1;
    }

    println!(
        "result after {} cycles: {}",
        cycle_bef,
        initial_grid
            .iter()
            .map(|l| l.iter().filter(|e| **e == '#').count())
            .sum::<usize>()
    );
}

fn divide_and_split(
    currrent_iteration: &[Vec<char>],
    mappings: &HashMap<&str, &str>,
) -> Vec<Vec<char>> {
    let intervals = if currrent_iteration.len() % 2 == 0 {
        2
    } else {
        3
    };

    let (mut i, mut j, mut ans) = (0, 0, vec![]);

    while i < currrent_iteration.len() {
        let mut tots = vec![];
        while j < currrent_iteration.len() {
            let slice = currrent_iteration
                .iter()
                .skip(i)
                .take(intervals)
                .map(|e| {
                    e.clone()
                        .into_iter()
                        .skip(j)
                        .take(intervals)
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<_>>();

            let combos = produce_rotations_and_flippations_d(&slice);
            let formatted_combos = combos
                .iter()
                .map(|comb| {
                    comb.iter()
                        .map(|f| f.iter().collect::<String>())
                        .collect::<Vec<_>>()
                        .join("/")
                })
                .collect::<Vec<_>>();

            for comb in &formatted_combos {
                if mappings.contains_key(comb.as_str()) {
                    tots.push(
                        mappings
                            .get(comb.as_str())
                            .unwrap()
                            .split('/')
                            .map(|r| r.chars().collect::<Vec<_>>())
                            .collect::<Vec<_>>(),
                    );
                    break;
                }
            }

            j += intervals;
        }
        // println!("tots: {:?}", tots);
        for y in 0..tots[0].len() {
            let mut temp = vec![];
            for mini in &tots {
                temp.extend(mini[y].iter());
            }
            ans.push(temp);
        }

        j = 0;
        i += intervals;
    }
    ans
}
