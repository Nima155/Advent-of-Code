use std::ops::Index;






fn main() {
    // 430 players; last marble is worth 71588 points
    let mut scores = vec![0; 430];


    let mut marbles = vec![];
    
    let (mut cur_marble, mut cur_indx) = (0, 0);

    'a: loop {
        for j in 0..430 {
            // println!("{:?} \n{}", marbles, cur_indx);
            match (cur_marble == 0, cur_marble % 23 == 0) {
                (true, _) => { marbles.push(0); }
                (_, true) => {
                    scores[j] += cur_marble;

                    

                    let to_rem;
                    if cur_indx < 7 {
                        to_rem = marbles.len() - (7 - cur_indx);
                    } else {
                        to_rem = cur_indx - 7;
                    }
                    
                    cur_indx = to_rem % marbles.len();
                    
                    let removed = marbles.remove(to_rem);
                    
                    scores[j] += removed;
                }
                _ => {
                    cur_indx = (cur_indx + 2) % marbles.len();
                    
                    marbles.insert(if marbles.len() == 1 {cur_indx + 1} else { cur_indx }, cur_marble);
                }
            }
            cur_marble += 1;
            if cur_marble > 71588 {
                break 'a;
            }
            
        }
    }
    // println!("{:?}", scores);
    println!("{}", scores.iter().max().unwrap());
}
