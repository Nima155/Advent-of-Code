use std::{collections::HashSet, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let points = lines
        .split("\r\n")
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    let mut visited = HashSet::new();
    for p in points.iter() {
        if !visited.contains(&(p[0], p[1], p[2], p[3])) {
            dfs(&points, &mut visited, p);
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn manhattan_distance(p1: &[i64], p2: &[i64]) -> i64 {
    p1.iter()
        .zip(p2.iter())
        .fold(0, |acc, cur| acc + (cur.0 - cur.1).abs())
}

fn dfs(points: &[Vec<i64>], visited: &mut HashSet<(i64, i64, i64, i64)>, cur_point: &[i64]) {
    let pp = (cur_point[0], cur_point[1], cur_point[2], cur_point[3]);

    if visited.contains(&pp) {
        return;
    }

    visited.insert(pp);

    for p in points {
        if p != cur_point && manhattan_distance(p, cur_point) <= 3 {
            dfs(points, visited, p);
        }
    }
}
