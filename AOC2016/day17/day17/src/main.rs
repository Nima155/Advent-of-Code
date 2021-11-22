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
    queue.push_back((STARTER.to_owned(), String::new(), (0, 0)));

    while !queue.is_empty() {
        let (current, path, (y, x)) = queue.pop_front().unwrap();
        if y == 3 && x == 3 {
            println!("{}", path);
            break;
        }
        visited.insert(current.clone());
        let actual_key = format!("{:0x}", md5::compute(&current))
            .chars()
            .collect::<Vec<_>>();

        let [up, down, left, right] = [
            format!("{}{}", current, 'U'),
            format!("{}{}", current, 'D'),
            format!("{}{}", current, 'L'),
            format!("{}{}", current, 'R'),
        ];

        if is_open(actual_key[0]) && !visited.contains(&up) && y > 0 {
            queue.push_back((up, format!("{}{}", path, 'U'), (y - 1, x)));
        }
        if is_open(actual_key[1]) && !visited.contains(&down) && y < 3 {
            queue.push_back((down, format!("{}{}", path, 'D'),  (y + 1, x)));
        }
        if is_open(actual_key[2]) && !visited.contains(&left) && x > 0 {
            queue.push_back((left, format!("{}{}", path, 'L'), (y , x - 1)));
        }
        if is_open(actual_key[3]) && !visited.contains(&right) && x < 3 {
            queue.push_back((right, format!("{}{}", path, 'R'), (y , x + 1)));
        }
    }
}
