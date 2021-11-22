use std::collections::{HashSet, VecDeque};

const STARTER: &str = "njfxhljp";
fn main() {
    traverse();
}
fn is_open(car: char) -> bool {
    ['b', 'c', 'd', 'e', 'f'].contains(&car)
}
fn traverse() {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((STARTER.to_owned(), 0, (0, 0)));
    let mut mx = 0;
    while !queue.is_empty() {
        let (current, p_l,  (y, x)) = queue.pop_front().unwrap();
        
        if y == 3 && x == 3 {
            mx = i32::max(mx, p_l);
            continue;
        }
        visited.insert(((y, x), p_l));
        let actual_key = format!("{:0x}", md5::compute(&current))
            .chars()
            .collect::<Vec<_>>();
        
        if is_open(actual_key[0]) && !visited.contains(&((y - 1, x), p_l + 1)) && y > 0 {
            queue.push_back((format!("{}{}", current, 'U'), p_l + 1, (y - 1, x)));
        }
        if is_open(actual_key[1]) && !visited.contains(&((y + 1, x), p_l + 1)) && y < 3 {
            // visited.insert(((y + 1, x), p_l + 1));
            queue.push_back((format!("{}{}", current, 'D'), p_l + 1,  (y + 1, x)));
        }
        if is_open(actual_key[2]) && !visited.contains(&((y , x - 1), p_l + 1)) && x > 0 {
            // visited.insert(((y, x - 1), p_l + 1));
            queue.push_back((format!("{}{}", current, 'L'), p_l + 1, (y , x - 1)));
        }
        if is_open(actual_key[3]) && !visited.contains(&((y , x + 1), p_l + 1)) && x < 3 {
            // visited.insert(((y, x + 1), p_l + 1));
            queue.push_back((format!("{}{}", current, 'R'), p_l + 1, (y , x + 1)));
        }
    }
    println!("longest path length: {}", mx);
}
