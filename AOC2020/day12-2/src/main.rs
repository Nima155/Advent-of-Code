use std::fs;
use std::iter::FromIterator;

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let (mut way_y, mut way_x) = (-1, 10);

    let (mut boat_y, mut boat_x) = (0, 0);

    read.split("\r\n").for_each(|e| {
        let chars = e.chars().collect::<Vec<char>>();
        let mut number_of_moves: i32 = String::from_iter(&chars[1..]).parse::<i32>().unwrap();
        // println!("{} {} {} {}", cur_dir_x, way_x, cur_dir_y, way_y);
        match chars[0] {
            'N' | 'S' | 'E' | 'W' => {
                way_y += if chars[0] == 'N' {
                    -number_of_moves
                } else {
                    number_of_moves * ((chars[0] == 'S') as i32)
                };

                way_x += if chars[0] == 'W' {
                    -number_of_moves
                } else {
                    number_of_moves * ((chars[0] == 'E') as i32)
                };
            }
            'L' | 'R' => {
                if number_of_moves != 360 {
                    number_of_moves *= if chars[0] == 'L' { -1 } else { 1 };

                    let number_of_moves = (number_of_moves as f64).to_radians();

                    let new_way_x = (way_x as f64 * (number_of_moves).cos()
                        - way_y as f64 * (number_of_moves).sin())
                    .round();

                    let new_way_y = (way_x as f64 * (number_of_moves).sin()
                        + way_y as f64 * (number_of_moves).cos())
                    .round();

                    

                    // let (oldx, oldy) = (way_x, way_y);

                    way_x = new_way_x as i32;
                    way_y = new_way_y as i32;

                    // if number_of_moves != 180 {
                    //     swap(&mut way_x, &mut way_y);
                    // }
                }
            }
            'F' => {
                boat_y += number_of_moves * way_y;
                boat_x += number_of_moves * way_x;
            }
            _ => {}
        }
    });
    println!("{} {}", way_x, way_y);
    println!("{}", boat_y.abs() + boat_x.abs(),);
}
