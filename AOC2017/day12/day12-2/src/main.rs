use std::{collections::{HashMap, HashSet}, fs};
fn main() {
    let lines = fs::read_to_string("./input.txt").unwrap(); 

    let map = lines.split("\r\n").map(|l| { 
        let args = l.split(" <-> ").collect::<Vec<_>>();
        (args[0], args[1].split(", ").collect::<Vec<_>>())
    }).collect::<HashMap<_, _>>();


    let mut visited = HashSet::new();
    let mut ans = 0;
    for root in map.keys() {
        ans += dfs(&map, &mut visited, root);
    }
    print!("{}", ans);

}

fn dfs<'a>(mappings: &'a HashMap<&str, Vec<&str>>, visited: &mut HashSet<&'a str>, cur_prog: &'a str) -> i64 {
    if visited.contains(cur_prog) {
        return 0;
    }

    visited.insert(cur_prog);
    

    for neigh in mappings.get(cur_prog).unwrap() {
        dfs(mappings, visited, *neigh);
    }

    1
}