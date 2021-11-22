const DESIRED_LENGTH: usize = 35651584; // swap value for part 1
fn main() {
    let mut state = "01000100010010111".to_owned();
    while state.len() < DESIRED_LENGTH {
        state = next_state(&state);
    }
    produce_checksum(&state[..DESIRED_LENGTH]);
}
fn next_state(state: &str) -> String {
    format!("{}{}{}", state, 0, state.chars().rev().map(|e| if e == '0' {'1'} else {'0'}).collect::<String>())
}
fn produce_checksum(state: &str)  {
    let mut state = state.chars().collect::<Vec<_>>();
    while state.len() % 2 == 0 {
        let mut i = 0;
        let mut new_state = Vec::new();
        while i < state.len() {
            new_state.push(if state[i] == state[i + 1] { '1' } else { '0' });
            i += 2;
        } 
        state = new_state;

    }
    println!("{}", state.iter().collect::<String>());
}