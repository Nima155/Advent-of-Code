use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let mut lines = lines
        .split("\r\n")
        .map(|l| l.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let length = lines.len();
    lines[..length].reverse();
    // println!("{:?}", lines);
    let mut pass = "fbgdceah".chars().collect::<Vec<_>>();
    println!("{}", descramble(&lines, &mut pass));
}

fn find_indx<T: Ord + PartialOrd>(array: &[T], char_to_find: T) -> usize {
    array
        .iter()
        .enumerate()
        .find(|(_, e)| **e == char_to_find)
        .unwrap()
        .0
}

fn descramble(instructions: &[Vec<&str>], unscrambled_password: &mut Vec<char>) -> String {
    for ins in instructions {
        let first_arg = ins[2];

        match ins[0] {
            "move" => {
                let rm = unscrambled_password.remove(ins.last().unwrap().parse::<usize>().unwrap());

                unscrambled_password.insert(first_arg.parse::<usize>().unwrap(), rm);
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
                    unscrambled_password.rotate_left(first_arg.parse::<usize>().unwrap());
                }
                "left" => {
                    unscrambled_password.rotate_right(first_arg.parse::<usize>().unwrap());
                }
                _ => {
                    let indx = find_indx(
                        unscrambled_password,
                        ins.last().unwrap().chars().next().unwrap(),
                    );
                    // this is will only work on passwords of size 8. i.e hardcoded
                    match indx {
                        0 => unscrambled_password.rotate_left(1),
                        1 | 3 | 5 | 7 => unscrambled_password.rotate_left(indx / 2 + 1),
                        2 => unscrambled_password.rotate_left(6),
                        4 => unscrambled_password.rotate_left(7),

                        _ => {}
                    }
                }
            },

            _ => {}
        }
    }
    unscrambled_password.iter().collect::<String>()
}
