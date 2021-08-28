use std::collections::HashMap;
fn main() {
    let input = [1,20,11,6,12,0];

    let mut to_map: HashMap<i32, usize> = input.iter().enumerate().map(|(i,e)| (*e, i + 1)).collect();

    let (mut turn, mut last) = (input.len() + 1, 0);
    

    while turn < 30000000 {
        
        match to_map.get_mut(&last) {
            Some(v) => { last = (turn - *v) as i32; *v = turn; },
            None => { to_map.insert(last, turn); last = 0; }
        }
        turn += 1;
    }
    println!("{}", last);
}
