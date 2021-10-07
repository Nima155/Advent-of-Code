use std::{collections::VecDeque, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let actions = lines.split("\r\n").map(|l| {
        match (l.ends_with("stack"), l.starts_with("cut")) {
            (true, _) => { "stack".to_owned()},
            (_, true) => { l.to_owned() }
            (false, false) => { format!("inc {}", l.split(' ').last().unwrap()) }
        }
    }).collect::<Vec<_>>();

    println!("{}", play_cards(&actions));

}

fn incrementer(cards: &[i32], gap: usize) -> VecDeque<i32> {

    let mut ans = VecDeque::from(vec![-1; cards.len()]);
    let (mut i, mut j) = (0, 0);
    
    while i < cards.len() {
        let indx = j % cards.len();
        if ans[indx] == -1 {
            ans[indx] = cards[i];
            i += 1;
        }
        j += gap;
    }


    ans
}


fn play_cards(actions: &[String]) -> usize {
    let mut cards = (0..10007).collect::<VecDeque<_>>();
    
    for act in actions {
        match (act.contains("stack"), act.contains("inc")) {
            (true, _) => {
                cards = cards.into_iter().rev().collect::<VecDeque<_>>();
            }
            (_, true) => {
                // println!("{}", act.split_at(3).1);
                cards = incrementer(cards.make_contiguous(), act.split_at(3).1.trim().parse().unwrap());
            }
            (_, _) => {
                let num = act.split_at(3).1.trim().parse::<i32>().unwrap();
                if num < 0 {
                    cards.rotate_right(num.abs() as usize);
                } else {
                    cards.rotate_left(num as usize);
                }

            }
        }
        // println!("{:?}", cards);    
    }
    
    
    cards.iter().position(|e| *e == 2019).unwrap()
}