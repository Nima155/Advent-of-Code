use std::collections::HashMap;

fn main() {
    println!("{:?}", most_wins(0, 0, 5, 4, 0, &mut HashMap::new()));
}
fn reducer(pos: u128) -> u128 {
    if pos % 10 == 0 {
        10
    } else {
        pos % 10
    }
}

type ExtremeMap = HashMap<(u128, u128, u128, u128, i8), (u128, u128)>;

fn most_wins(
    p1_score: u128,
    p2_score: u128,
    p2_pos: u128,
    p1_pos: u128,
    turn: i8,
    cache: &mut ExtremeMap,
) -> (u128, u128) {
    let key = (p1_score, p2_score, p2_pos, p1_pos, turn);

    if cache.contains_key(&key) {
        return cache[&key];
    }

    if p1_score >= 21 {
        return (1, 0);
    } else if p2_score >= 21 {
        return (0, 1);
    }

    let (mut a, mut b) = (0, 0);
    // let mut visited = HashSet::new();
    for i in 1..=3 {
        for j in 1..=3 {
            for v in 1..=3 {
                let intermed;
                // let mut draw = [i,  j, v];
                // if !visited.contains(&draw) {
                if turn == 1 {
                    let p2_pos = reducer(p2_pos + i + j + v);

                    intermed =
                        most_wins(p1_score, p2_score + p2_pos, p2_pos, p1_pos, turn ^ 1, cache);
                } else {
                    let p1_pos = reducer(p1_pos + i + j + v);

                    intermed =
                        most_wins(p1_score + p1_pos, p2_score, p2_pos, p1_pos, turn ^ 1, cache);
                }
                a += intermed.0;
                b += intermed.1;
            }
        }
    }
    cache.insert(key, (a, b));
    (a, b)
}
