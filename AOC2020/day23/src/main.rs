mod game;
use std::collections::VecDeque;

use crate::game::play_the_game;

fn main() {
    // 389125467
    let mut mock_input = VecDeque::from(vec![3,8,9,1,2,5,4,6,7]);
    let mut input = VecDeque::from(vec![3, 9, 8, 2, 5, 4, 7, 1, 6]);
    play_the_game(&mut input);
    // println!("{:?}", input);
    let mut some_split = input.split_off(input.iter().position(|x| *x==1).unwrap() + 1);
    some_split.extend(input.iter().take_while(|x| **x != 1));

    println!("{:?}", some_split);
}
