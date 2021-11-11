use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    
    let lines = fs::read_to_string("../input.txt").unwrap();

    /* 
     * parsing the input into [node, [children]] into a hashmap
    */
    let mapped = lines.split("\r\n").map(|l| {
        if let [parent, children] = l.split(" -> ").collect::<Vec<_>>()[..] {
            if let [parent_name, _] = parent.split(' ').collect::<Vec<_>>()[..] {
                return (parent_name, children.split(", ").collect::<Vec<_>>());
            }
        }
        ("you lose", vec![])
    }).collect::<HashMap<_, _>>();
    

    let (mut children, mut visited) = (HashSet::new(), HashSet::new());
    
    
    for (node, _) in &mapped {
        dfs(node, &mapped, &mut children, &mut visited);
    }

    print!("{:?}", visited.difference(&children));
}

fn dfs<'a>(cur_node: &'a str, mapped: &'a HashMap<&'a str, Vec<&'a str>>, children: &mut HashSet<&'a str>, visited: &mut HashSet<&'a str>) {
    if visited.contains(cur_node) || !mapped.contains_key(cur_node) {
        return;
    }
    
    visited.insert(cur_node);

    

    for k in mapped.get(cur_node).unwrap() { 
        children.insert(k);
        dfs(k, mapped, children, visited);
    }

    
}