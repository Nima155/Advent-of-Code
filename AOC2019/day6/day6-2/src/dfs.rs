use std::collections::{HashMap, HashSet};

pub fn dfs<'a>(
    mappings: &HashMap<&'a str, Vec<&'a str>>,
    cur_node: &'a str,
    visited: &mut HashSet<&'a str>,
) -> (bool, u64) {
    

    if visited.contains(cur_node) {
        return (false, 0);
    }
    visited.insert(cur_node);

    if mappings.contains_key(cur_node) {
        for node in mappings.get(cur_node).unwrap() {
            if node == &"SAN" {
                return (true, 0);
            }
            let (hit_targ, num) = dfs(mappings, node, visited);

            if hit_targ {
                return (true, 1 + num);
            }
        }
    }
    (false, 0)
}
