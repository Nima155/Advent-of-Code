use std::collections::{HashSet, VecDeque};

pub fn build_state(decks: &[VecDeque<u64>]) -> String {
    decks
        .iter()
        .map(|deck| {
            deck.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .collect::<Vec<_>>()
        .join("  ")
}

pub fn print_deck_score(player: &VecDeque<u64>) -> u64 {
    player
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, cur)| (i + 1) as u64 * cur + acc)
}

pub fn play_the_long_game(
    players: &mut Vec<VecDeque<u64>>,
    previous_deck_states: &mut HashSet<String>,
) -> (u64, u64) {
    while !players[0].is_empty() && !players[1].is_empty() {
        let state = build_state(players);

        if let true = previous_deck_states.contains(&state) {
            // println!("Hi");
            return (print_deck_score(&players[0]), print_deck_score(&players[1]));
        }

        previous_deck_states.insert(state);

        let (p1, p2) = (
            players[0].pop_front().unwrap(),
            players[1].pop_front().unwrap(),
        );

        match (p1 <= players[0].len() as u64, p2 <= players[1].len() as u64) {
            (true, true) => {
                let mut players_n = players
                    .iter()
                    .zip([p1, p2])
                    .map(|f| {
                        f.0.iter()
                            .take(f.1 as usize).cloned()
                            .collect::<VecDeque<_>>()
                    })
                    .collect::<Vec<_>>();

                play_the_long_game(&mut players_n, &mut HashSet::new());

                match players_n[0].is_empty() {
                    true => {
                        players[1].push_back(p2);
                        players[1].push_back(p1);
                    }
                    _ => {
                        players[0].push_back(p1);
                        players[0].push_back(p2);
                    }
                }
                // println!("{:?}", players);
            }
            _ => match p1 < p2 {
                true => {
                    players[1].push_back(p2);
                    players[1].push_back(p1);
                }
                _ => {
                    players[0].push_back(p1);
                    players[0].push_back(p2);
                }
            },
        }
    }
    (print_deck_score(&players[0]), print_deck_score(&players[1]))
}
