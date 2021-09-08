use std::collections::VecDeque;


pub fn find_new_dest(mut cur: i32, circle: &VecDeque<i32>, picked: &VecDeque<i32>) -> usize {
    loop {
        cur -= 1;
        if cur == 0 {
            cur = 9;
        }
        if !picked.contains(&cur) {
            return circle.iter().position(|x| *x == cur).unwrap()
        }

    }
}

pub fn play_the_game(circle: &mut VecDeque<i32>) {
    let (mut cur, mut cur_indx, mut cycle) = (circle[0], 0, 0);
    

    while cycle < 100 {
        let mut vec = circle
            .drain(cur_indx + 1..usize::min(cur_indx + 4, circle.len()))
            .collect::<VecDeque<_>>();

        while vec.len() < 3 {
            vec.push_back(circle.pop_front().unwrap());
        }
        let mut n_dest_indx = find_new_dest(cur, circle, &vec);
        
        while !vec.is_empty() {
            
            circle.insert(n_dest_indx + 1, vec.pop_front().unwrap());
            n_dest_indx += 1;

            
        }
        cur_indx = (circle.iter().position(|x| *x == cur).unwrap() + 1) % circle.len();
        cur = circle[cur_indx];
        cycle += 1;
        
    }
    
}
