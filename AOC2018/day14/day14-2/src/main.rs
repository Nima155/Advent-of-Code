fn main() {
    let mut initial = vec![3_usize, 7];

    let (mut minion_1, mut minion_2) = (0, 1);


    while initial.len() < 409561 {
        let new_recipe_parts =  (initial[minion_1] + initial[minion_2]).to_string();

        for dig in new_recipe_parts.chars() {
            initial.push(dig.to_digit(10).unwrap() as usize);
        }
        minion_1 = (minion_1 + initial[minion_1]  + 1) % initial.len();
        minion_2 = (minion_2 + initial[minion_2] + 1)  % initial.len();
        
    }
    println!("{:?}", &initial[409551..]);
}
