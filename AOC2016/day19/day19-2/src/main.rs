fn main() {
    simulate_process(3017957);
}

fn simulate_process(participants: usize) {
    let even_participants = if participants % 2 != 0 {
        participants - 1
    } else {
        participants
    };

    let mut infected_list = vec![0; even_participants];
    let mut infected_pointer = even_participants / 2;
    let mut infected = 0;

    while infected < (even_participants - 1) {
        infected += (infected_list[infected_pointer % even_participants] != 1) as usize;
        infected_list[infected_pointer % even_participants] = 1;
        infected_pointer += if (even_participants - infected) % 2 == 0 {
            2
        } else {
            1
        };
    }
    let ans = infected_list
        .iter()
        .enumerate()
        .find(|(_i, e)| **e == 0)
        .unwrap()
        .0;

    println!(
        "{}",
        if even_participants == participants {
            ans
        } else {
            ans + 2
        }
    )
}
