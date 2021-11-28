use std::fs;
const ROWS: usize = 25;
const COLUMNS: usize = 36;

fn main() {
    let nodes = fs::read_to_string("../input.txt").unwrap();

    let mut grid = [['.'; COLUMNS]; ROWS];
    nodes.split("\r\n").skip(2).for_each(|l| {
        let node_info = l.split_ascii_whitespace().collect::<Vec<_>>();
        let mut node_pos = node_info[0]
            .split('-')
            .skip(1)
            .map(|e| e[1..].parse::<usize>().unwrap());
        let [x, y] = [node_pos.next().unwrap(), node_pos.next().unwrap()];

        let used = strip_and_parse(node_info[2], 'T');

        if used == 0 {
            grid[y][x] = '_';
        } else if used > 200 {
            grid[y][x] = '#';
        } else if y == 0 {
            grid[y][x] = if x == COLUMNS - 1 {
                'G'
            } else if x == 0 {
                'S'
            } else {
                '.'
            };
        }
    });

    println!(
        "{}",
        grid.iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\r\n")
    ); // done without code!
       /*
          Count the number of places to the target node
          then count + (distance_to_origin - 1) * 5 + 1
       */
}

fn strip_and_parse(num: &str, car_strip: char) -> i64 {
    num.strip_suffix(car_strip).unwrap().parse::<i64>().unwrap()
}
