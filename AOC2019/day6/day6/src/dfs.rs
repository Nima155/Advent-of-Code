use std::collections::HashMap;

pub fn dfs<'a>(mappings: &HashMap<&'a str, &'a str>, cur_node: &'a str) -> u64 {
    let mut ans = 0;

    if mappings.contains_key(cur_node) {
        ans += 1 + dfs(mappings, mappings.get(cur_node).unwrap());
    }

    ans
}
