use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines.split("\r\n").collect::<Vec<_>>();

    let mut display = [['0'; 50]; 6];

    for instruction in lines {
        let split_it = instruction.split(' ').collect::<Vec<_>>();
        let by = split_it[split_it.len() - 1];
        match (instruction.starts_with("rotate r"), instruction.starts_with("rect")) {
            (true, _) => {
               display[split_it[2][2..].parse::<usize>().unwrap()].rotate_right(by.parse::<usize>().unwrap())
            }
            (_, true) => {
                let nums = by.split('x').map(|e| e.parse::<usize>().unwrap()).collect::<Vec<_>>();
                for n in 0..nums[0] {
                    for m in 0..nums[1] {
                        display[m][n] = '1';
                    }                
                }
            }
            (_, _) => {
                let col_num = split_it[2][2..].parse::<usize>().unwrap();
                let mut col = display.iter().map(|r| r[col_num]).collect::<Vec<_>>();
                col.rotate_right(by.parse::<usize>().unwrap());
                let mut iter = col.iter();

                display.iter_mut().for_each(|r| {
                    r[col_num] = *iter.next().unwrap();
                })
            }
        }
    }
    println!("{}", display.iter().map(|r| r.iter().collect::<String>()).collect::<Vec<_>>().join("\n")); // part 2
    println!("{}", display.iter().flatten().filter(|e| **e == '1').count()); // part 1
}
