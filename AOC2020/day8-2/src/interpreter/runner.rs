use std::collections::HashSet;

pub fn run_through(vec: Vec<Vec<&str>>) -> (bool, i32) {
    let mut acc = 0;
    let mut cur_indx: i32 = 0;

    let mut visited = HashSet::new();
    let length = vec.len();

    while (cur_indx as usize) != length {
        let instruction = &vec[cur_indx as usize];

        if visited.contains(&cur_indx) || cur_indx < 0 || (cur_indx as usize) > length {
            return (false, 0);
        }

        visited.insert(cur_indx);

        match instruction[0] {
            "acc" => {
                acc += instruction[1].parse::<i32>().unwrap();
                cur_indx += 1
            }
            "jmp" => cur_indx += instruction[1].parse::<i32>().unwrap(),
            "nop" => {
                cur_indx += 1;
            }
            _ => {}
        }
        
    }
    (true, acc)
}