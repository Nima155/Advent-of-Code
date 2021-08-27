use std::fs;
// Only works when all the numbers in the input are prime numbers!
fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let to_vec = read
        .split(',')
        .enumerate()
        .filter_map(|(i, el)| match el.parse::<u64>() {
            Ok(e) => Some((i, e)),
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    let mut time: u64 = 0;
    let mut step = to_vec[0].1;
    let mut offset: u64;
    

    for z in to_vec.iter().skip(1){
        offset = z.0 as u64;

        while (time + step + offset) % z.1 != 0 {
            time += step;
        }

        time += step;
        step *= z.1;
    }
    println!("{}", time);
}
