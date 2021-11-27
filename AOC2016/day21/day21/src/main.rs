use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines
        .split("\r\n")
        .map(|l| l.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pass = "abcdefgh".chars().collect::<Vec<_>>();
    println!("{}", scrambler(&lines, &mut pass));
}

fn find_indx<T: Ord + PartialOrd>(array: &[T], char_to_find: T) -> usize {
    array
        .iter()
        .enumerate()
        .find(|(_, e)| **e == char_to_find)
        .unwrap()
        .0
}

fn scrambler(instructions: &[Vec<&str>], unscrambled_password: &mut Vec<char>) -> String {
    for ins in instructions {
        let first_arg = ins[2];

        match ins[0] {
            "move" => {
                let rm = unscrambled_password.remove(first_arg.parse::<usize>().unwrap());
                
                unscrambled_password.insert(ins.last().unwrap().parse::<usize>().unwrap(), rm);
                
            }
            "swap" => match ins[1] {
                "position" => {
                    let indx = ins.last().unwrap().parse::<usize>().unwrap();
                    let indx_1 = ins[2].parse::<usize>().unwrap();
                    unscrambled_password.swap(indx, indx_1);
                }
                "letter" => {
                    let let_1_position =
                        find_indx(unscrambled_password, first_arg.chars().next().unwrap());
                    let let_2_position = find_indx(
                        unscrambled_password,
                        ins.last().unwrap().chars().next().unwrap(),
                    );
                    unscrambled_password.swap(let_1_position, let_2_position);
                }
                _ => {}
            },
            "reverse" => {
                let slice = &mut unscrambled_password[first_arg.parse::<usize>().unwrap()
                    ..ins.last().unwrap().parse::<usize>().unwrap() + 1];
                slice.reverse();
            }
            "rotate" => match ins[1] {
                "right" => {
                    unscrambled_password.rotate_right(first_arg.parse::<usize>().unwrap());
                }
                "left" => {
                    unscrambled_password.rotate_left(first_arg.parse::<usize>().unwrap());
                }
                _ => {
                    let indx = find_indx(
                        unscrambled_password,
                        ins.last().unwrap().chars().next().unwrap(),
                    );
                    let additional = if indx >= 4  { 1 } else { 0 };
                    let length = unscrambled_password.len();
                    unscrambled_password.rotate_right((1 + additional + indx) % length);
                    
                }
            },

            _ => {}
        }
        
    }
    unscrambled_password.iter().collect::<String>()
}
