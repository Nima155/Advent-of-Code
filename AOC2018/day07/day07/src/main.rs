#![feature(map_first_last)]

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let mut job_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut initial_nodes = BTreeSet::new();

    for l in lines.split("\r\n") {
        let stuff = l.split(' ').collect::<Vec<_>>();

        let (job_bef, job_aft) = (stuff[1], stuff[7]);

        let entry = job_map.entry(job_aft).or_default();

        entry.insert(job_bef);
        initial_nodes.insert(job_aft);
        initial_nodes.insert(job_bef);
    }
    initial_nodes = initial_nodes
        .into_iter()
        .filter(|e| !job_map.contains_key(*e))
        .collect();
    // println!("{:?} \n{:?}", job_map, initial_nodes);
    println!("{}", topological_sort(&mut initial_nodes, &mut job_map));

    // 1 and 7... one must be done before seven
}

fn topological_sort<'a>(
    initial_nodes: &'a mut BTreeSet<&'a str>,
    job_map: &'a mut HashMap<&str, HashSet<&str>>,
) -> String {
    let mut ans = String::new();
    let mut visited = HashSet::new();
    while !initial_nodes.is_empty() {
        let node = initial_nodes.pop_first().unwrap();
        ans.push_str(node);
        for (n, _) in job_map.clone() {
            job_map.get_mut(n).unwrap().remove(node);
            if job_map.get(n).unwrap().is_empty() && !visited.contains(n) {
                initial_nodes.insert(n);
                visited.insert(n);
            }
        }
    }
    ans
}
