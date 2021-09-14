mod play_the_numbers;
use std::fs;

use play_the_numbers::{determine_final_image, layer_cake};

fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let to_digits = read
        .chars()
        .map(|f| f.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let cake = layer_cake(25, 6, &to_digits);

    determine_final_image(&cake);
}
