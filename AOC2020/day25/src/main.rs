use std::fs;
fn main() {
    let read =  fs::read_to_string("input.txt").unwrap();

    // Set the value to itself multiplied by the subject number. => number 7 ?!
    // Set the value to the remainder after dividing the value by 20201227.

    let vector = read.split("\r\n").map(|f| f.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let loop_size = time_to_play_the_game(&vector);


    for (indx, i) in vector.iter().enumerate() {
        let mut i_copy = 1;
        
        for _ in 0..loop_size[indx] {
            i_copy *= vector[if indx >= 1 { indx - 1 } else { indx + 1 }];
            i_copy %= DIV_BY;
            
        }
        println!("{}", i_copy);
    }

}
const SUBJECT_NUMBER: u64 = 7;
const DIV_BY: u64 = 20201227;


fn time_to_play_the_game(vector: &Vec<u64>) -> Vec<i32> {
    let mut vec_loop_sizes = Vec::new();
    for i in vector {
        let mut i_copy = 1;
        let mut loop_size = 0;
        while i_copy != *i {
            i_copy *= SUBJECT_NUMBER;
            i_copy %= DIV_BY;
            loop_size += 1;
        }
        vec_loop_sizes.push(loop_size);
    }
    vec_loop_sizes
}