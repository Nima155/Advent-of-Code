use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    /*
     * parsing the input into [node, [children]] into a hashmap
     */
    let mapped = lines
        .split("\r\n")
        .map(|l| {
            if let [parent, children] = l.split(" -> ").collect::<Vec<_>>()[..] {
                let vec = parent.split(' ').collect::<Vec<_>>();
                return (
                    (vec[0], vec[1][1..vec[1].len() - 1].parse::<i64>().unwrap()),
                    children.split(", ").collect::<Vec<_>>(),
                );
            } else {
                let v = l.split(' ').collect::<Vec<_>>();
                (
                    (v[0], v[1][1..v[1].len() - 1].parse::<i64>().unwrap()),
                    vec![],
                )
            }
        })
        .collect::<HashMap<_, _>>();

    let (mut children, mut visited) = (HashSet::new(), HashMap::new());
    // println!("{:#?}", mapped);
    for node in mapped.keys() {
        dfs(*node, &mapped, &mut children, &mut visited);
    }
}

fn dfs<'a>(
    cur_node: (&'a str, i64),
    mapped: &'a HashMap<(&'a str, i64), Vec<&'a str>>,
    children: &mut HashSet<&'a str>,
    visited: &mut HashMap<&'a str, i64>,
) -> i64 {
    if visited.contains_key(cur_node.0) {
        return *visited.get(cur_node.0).unwrap();
    }

    let mut ans: HashMap<i64, Vec<_>> = HashMap::with_capacity(10);

    for k in mapped.get(&cur_node).unwrap() {
        children.insert(k);

        let child_sum = dfs(
            mapped.keys().copied().find(|e| e.0 == *k).unwrap(),
            mapped,
            children,
            visited,
        );

        let entry = ans.entry(child_sum).or_default();
        entry.push(*k);
    }

    let total_sum = ans.iter().map(|(a, b)| a * b.len() as i64).sum::<i64>();

    if !ans.iter().all(|e| e.0 == ans.keys().next().unwrap()) {
        let ele_min = ans
            .iter()
            .min_by(|(_, a), (_, b)| a.len().cmp(&b.len()))
            .unwrap();
        let ele_max = ans
            .iter()
            .max_by(|(_, a), (_, b)| a.len().cmp(&b.len()))
            .unwrap();

        /*
         * The first number that gets printed onto the console is the answer
         */
        println!(
            "{:?}",
            mapped.iter().find(|e| e.0 .0 == ele_min.1[0]).unwrap().0 .1 - (ele_min.0 - ele_max.0)
        );
    }
    visited.insert(cur_node.0, cur_node.1 + total_sum);

    total_sum + cur_node.1
}
