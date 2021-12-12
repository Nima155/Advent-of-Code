use std::{
    collections::{HashMap, VecDeque},
    fs,
};
fn main() {
    let connections = fs::read_to_string("../test.txt").unwrap();
    let connections = connections
        .split("\r\n")
        .map(|l| l.split('-').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut connections_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for mappings in &connections {
        if let [src, dest] = mappings[..] {
            let entry = connections_map.entry(src).or_default();
            entry.push(dest);
            let entry_2 = connections_map.entry(dest).or_default();
            entry_2.push(src);
        }
    }
    println!(
        "{}",
        bfs(
            &connections_map,
            &connections_map.keys().copied().collect::<Vec<&str>>()
        )
    )
}

fn bfs(connections: &HashMap<&str, Vec<&str>>, connection_indices: &[&str]) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((
        "start",
        1 << connection_indices
            .iter()
            .position(|l| *l == "start")
            .unwrap(),
    ));

    let mut ans = 0;
    while !queue.is_empty() {
        let (on, visited) = queue.pop_front().unwrap();
        if on == "end" {
            ans += 1;
            continue;
        }
        for neigh in &connections[on] {
            let indx_of = connection_indices.iter().position(|l| l == neigh).unwrap();
            if (1 << indx_of & visited) == 0 || neigh.chars().next().unwrap().is_uppercase() {
                queue.push_back((neigh, visited | (1 << indx_of)));
            }
        }
    }
    ans
}
