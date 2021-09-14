mod dfs;
use std::{collections::HashMap, error::Error, fs};

use dfs::dfs;
fn main() -> Result<(), Box<dyn Error>> {
    let read = fs::read_to_string("../input.txt")?;

    let mut mappings: HashMap<&str, &str> = HashMap::new();

    for l in read.split("\r\n") {
        let (orbitee, orbiter) = l.split_at(3);
        mappings.entry(&orbiter[1..]).or_insert(orbitee);
    }
    let mut ans = 0;
    for node in mappings.keys() {
        ans += dfs(&mappings, node);
    }
    println!("{}", ans);

    Ok(())
}
