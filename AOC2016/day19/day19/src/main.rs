
const PARTICIPANTS: usize = 3017957;

fn main() {
    let mut participants = Vec::with_capacity(PARTICIPANTS);
    for i in 0..PARTICIPANTS {
        participants.push(i + 1);
    }

    while participants.len() > 1 {
        let mut new_participants = vec![];
        let mut i = if participants.len() % 2 == 0 { 0 } else { 2 };
        while i < participants.len() {
            new_participants.push(participants[i]);
            i += 2;
        }
        participants = new_participants;
        
    }
    println!("{:?}", participants[0]);
}
