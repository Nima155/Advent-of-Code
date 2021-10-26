use std::fs;
fn main() {
    let grid_str = fs::read_to_string("../input.txt").unwrap();

    let mut grid = grid_str.split("\r\n").map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();


    // | => tree, open => ., lumber => #
    println!("{}", cycle_field(&mut grid, 100000));
}



fn cycle_field(grid: &mut Vec<Vec<char>>, mut cycles: i64) -> i64 {

    while cycles > 0 {
        let mut grid_clone = grid.clone();    
        for i in 0..grid.len() {            
            for j in 0..grid[i].len() {
                let neighs = find_neighbors(grid, (i, j));
                
                let cur_char = grid[i][j];

                match cur_char {
                    '.' if neighs[2] >= 3 => {  grid_clone[i][j] = '|'  }
                    '|' if neighs[1] >= 3 => {  grid_clone[i][j] = '#'  }
                    '#' if neighs[1] < 1 || neighs[2] < 1 =>   { grid_clone[i][j] = '.' }
                    _ => { }
                }

            }
        }
        *grid = grid_clone;

        cycles -= 1;
        let flattened = grid.iter().flatten().collect::<Vec<_>>();
        println!("{}", (flattened.iter().filter(|e| ***e == '#').count() * flattened.iter().filter(|e| ***e == '|').count()) as i64);
    }
    let flattened = grid.iter().flatten().collect::<Vec<_>>();

    (flattened.iter().filter(|e| ***e == '#').count() * flattened.iter().filter(|e| ***e == '|').count()) as i64
}

fn find_neighbors(grid: &[Vec<char>], point: (usize, usize)) -> [i64; 3]  {
    let mut ans = [0; 3];
    for i in -1..=1 {
        for j in -1..=1 {
            let (ny, nx) = (point.0 as i64 + i, point.1 as i64 + j);
            if (i != 0 || j != 0) && ny < grid.len() as i64 && ny >= 0 && nx >= 0 && nx < grid[ny as usize].len() as i64 {
                match grid[ny as usize][nx as usize] {
                    '.' => { ans[0] += 1 }
                    '#' => { ans[1] += 1 }
                    '|' => { ans[2] += 1 }
                    _ => {}
                }
            }
        }
    }
    ans
}