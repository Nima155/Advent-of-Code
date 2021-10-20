fn main() {
    let mut initial = vec![3_usize, 7];

    let (mut minion_1, mut minion_2) = (0, 1);


    'a: loop {
        let new_recipe_parts =  (initial[minion_1] + initial[minion_2]).to_string();
        
        for dig in new_recipe_parts.chars() {
            initial.push(dig.to_digit(10).unwrap() as usize);
            if initial.len() >= 6 && initial[initial.len() - 6..] == [4,0,9,5,5,1] {
                println!("{}", initial.len() - 6);
                break 'a;
            }
        }
        minion_1 = (minion_1 + initial[minion_1]  + 1) % initial.len();
        minion_2 = (minion_2 + initial[minion_2] + 1)  % initial.len();

    }
    // println!("{:?}", &initial[409551..]);
}
