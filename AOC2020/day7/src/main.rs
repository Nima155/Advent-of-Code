use std::collections::{HashMap, HashSet};
use std::fs;
// macro_rules! new_map {
//     ($($k:expr => $v:expr),* $(,)?) => {{
//         let mut map = HashMap::new();
//         $(map.insert($k, $v);)+
//         map
//     }}
// }

fn dfs<'a>(
    c: &'a str,
    bags: &'a HashMap<String, Option<HashMap<String, i32>>>,
    mut visited: HashSet<&'a str>,
    mut memo: HashMap<&'a str, i32>,
) -> (i32, HashSet<&'a str>, HashMap<&'a str, i32>) {
    if c == "shiny gold" {
        return (1, visited, memo);
    }

    if memo.contains_key(c) {
        return ((*memo.get(c).unwrap() > 0) as i32, visited, memo);
    }

    let mut ans = 0;

    visited.insert(c);

    if let Some(cols) = bags.get(c) {
        for inner in cols {
            for col in inner.keys() {
                let (num, vis, cache) = dfs(col, bags, visited, memo);

                ans |= num;
                memo = cache;
                visited = vis;
            }
        }
    };

    memo.insert(c, ans);
    (ans, visited, memo)
}

fn main() {
    // let bags = new_map!("lr" => new_map!("bw" => 1, "my" => 2),
    // "do" => new_map!("bw" => 3, "my" => 4),
    // "bw" => new_map!("sg" => 1), "my" => new_map!("sg" => 2, "fb" => 9),
    // "sg" => new_map!("dol" => 1, "vp" => 2), "dol" => new_map!("fb" => 3, "db" => 4),
    // "vp" => new_map!("fb" => 5, "db"=> 6));
    let read = fs::read_to_string("input.txt").unwrap();

    let mut visited: HashSet<&str> = HashSet::new();
    let mut ans = 0;
    let mut cache: HashMap<&str, i32> = HashMap::new();

    let bags: HashMap<String, _> = read
        .split("\r\n")
        .flat_map(|line| {
            let bag_before_processing = line.replace("contain", ",");
            let bag = bag_before_processing.split(", ").collect::<Vec<_>>();
            let mother_bag = bag[0].split(' ').take(2).collect::<Vec<_>>().join(" ");

            let mut intermediate_map = HashMap::new();
            intermediate_map.insert(mother_bag.clone(), Some(HashMap::new()));

            bag.iter().skip(1).for_each(|other_bag| {
                let other_bag_disect = other_bag.split(' ').collect::<Vec<&str>>();

                if let Ok(z) = other_bag_disect.first().unwrap().parse() {
                    intermediate_map
                        .get_mut(&mother_bag)
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .insert(other_bag_disect[1..other_bag_disect.len() - 1].join(" "), z);
                }
            });
            if intermediate_map
                .get_mut(&mother_bag)
                .unwrap()
                .as_mut()
                .unwrap()
                .is_empty()
            {
                intermediate_map.insert(mother_bag, None);
            }
            // println!("{:?}", intermediate_map);
            intermediate_map
        })
        .collect();

    for (k, _val) in bags.iter() {
        if !cache.contains_key(k.as_str()) && *k != "shiny gold" {
            let (i, vis, memo) = dfs(k.as_str(), &bags, visited, cache);
            cache = memo;
            visited = vis;
            ans += i;
        } else if cache.contains_key(k.as_str()) {
            ans += cache.get(k.as_str()).unwrap();
        }
    }
    println!("{}", ans);
}
