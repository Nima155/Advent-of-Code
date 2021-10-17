use dlv_list::VecList;

fn main() {
    // 430 players; last marble is worth 71588 points
    let mut scores = vec![0; 430];

    let mut marbles: VecList<i128> = VecList::new();

    let (mut cur_marble, mut cur_indx) = (0, None);

    'a: loop {
        for j in 0..430 {
            // println!("{:?}", marbles);
            // println!("{:?} \n{}", marbles, cur_indx);
            // if cur_marble > 50000 {println!("{}", cur_marble);}
            match (cur_marble == 0, cur_marble % 23 == 0) {
                (true, _) => {
                    cur_indx = Some(marbles.push_back(0));
                }
                (_, true) => {
                    scores[j] += cur_marble;
                    let mut moves = 0;
                    while moves < 7 {
                        cur_indx = marbles.get_previous_index(cur_indx.unwrap());
                        if cur_indx.is_none() {
                            let indx = marbles.push_back(0);
                            cur_indx = marbles.get_previous_index(indx);

                            marbles.pop_back();
                        }
                        moves += 1;
                    }

                    let next_indx = marbles.get_next_index(cur_indx.unwrap());
                    // let to_rem;
                    // if cur_indx < 7 {
                    //     to_rem = marbles.len() - (7 - cur_indx);
                    // } else {
                    //     to_rem = cur_indx - 7;
                    // }

                    // cur_indx = to_rem % marbles.len();

                    let removed = marbles.remove(cur_indx.unwrap()).unwrap();
                    cur_indx = next_indx;
                    scores[j] += removed;
                }
                _ => {
                    let mut steps = 0;
                    while steps < 2 {
                        if steps == 1 && marbles.get_next_index(cur_indx.unwrap()).is_none() {
                            break;
                        }
                        cur_indx = marbles.get_next_index(cur_indx.unwrap());
                        if cur_indx.is_none() {
                            let indx = marbles.push_front(0);
                            cur_indx = marbles.get_next_index(indx);
                            marbles.pop_front();
                        }
                        steps += 1;
                    }
                    let other_indx = cur_indx.unwrap();
                    // println!("{:?}", marbles);
                    if marbles.len() == 1 || steps == 1 {
                        cur_indx = Some(marbles.insert_after(other_indx, cur_marble));
                    } else {
                        cur_indx = Some(marbles.insert_before(other_indx, cur_marble));
                    }
                }
            }
            cur_marble += 1;
            if cur_marble > (71588 * 100) {
                break 'a;
            }
        }
    }
    // println!("{:?}", scores);
    println!("{}", scores.iter().max().unwrap());
}
