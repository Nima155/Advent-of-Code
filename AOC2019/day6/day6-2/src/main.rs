mod dfs;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

use dfs::dfs;
fn main() -> Result<(), Box<dyn Error>> {
    let read = fs::read_to_string("../input.txt")?;

    let mut mappings: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in read.split("\r\n") {
        let orbitos = l.split(')').collect::<Vec<_>>();
        let mp = mappings.entry(orbitos[0]).or_insert(vec![orbitos[1]]);
        if mp[0] != orbitos[1] {
            mp.push(orbitos[1])
        }
        let mp2 = mappings.entry(orbitos[1]).or_insert(vec![orbitos[0]]);
        if mp2[0] != orbitos[0] {
            mp2.push(orbitos[0])
        }
    }

    println!("{}", dfs(&mappings, "YOU", &mut HashSet::new()).1 - 1);

    Ok(())
}
