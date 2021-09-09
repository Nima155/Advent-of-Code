mod game;

use indexmap::IndexMap;

use crate::game::play_the_game;

fn main() {
    let mut arr = vec![3, 9, 8, 2, 5, 4, 7, 1, 6];
    arr.extend(10..1000001);

    let mut mappings = IndexMap::new();

    for (indx, num) in arr.iter().enumerate() {
        mappings.insert(
            *num,
            if indx + 1 < arr.len() {
                arr[indx + 1]
            } else {
                arr[0]
            },
        ); // num + at most 1 neighbor
    }

    play_the_game(&mut mappings);

    println!(
        "{}",
        *mappings.get(&1).unwrap() as u128
            * *mappings.get(mappings.get(&1).unwrap()).unwrap() as u128
    );
}
