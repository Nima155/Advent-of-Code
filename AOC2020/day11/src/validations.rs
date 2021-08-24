const MOVES: [[i8;2];8] = [[1,0],[-1,0],[0,1],[0,-1],[1,1],[1,-1],[-1,1],[-1,-1]];

// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
// Otherwise, the seat's state does not change.


pub fn validate_empty_seat(grid: &Vec<Vec<char>>, i: usize, j: usize, marked: &mut Vec<[usize;2]>) -> bool {

    for [y, x] in MOVES.iter() {
        let (ny, nx) = (i as i64 + *y as i64, j as i64 + *x as i64);

        if ny >= 0 && ny < grid.len() as i64 && nx >= 0 && nx < grid[i].len() as i64 {
            if grid[ny as usize][nx as usize] == '#' {
                return false;
            }
        }
    }
    marked.push([i, j]);

    true
}

pub fn validate_occupied_seat(grid: &Vec<Vec<char>>, i: usize, j: usize, marked: &mut Vec<[usize;2]>) -> bool {
    let mut occs = 0;
    for [y, x] in MOVES.iter() {
        let (ny, nx) = (i as i64 + *y as i64, j as i64 + *x as i64);

        if ny >= 0 && ny < grid.len() as i64 && nx >= 0 && nx < grid[i].len() as i64 {
            if grid[ny as usize][nx as usize] == '#' {
                occs += 1;    
            }
        }
    }
    if occs >= 4 {
        marked.push([i, j]);
        return true;
    }
    false
}

pub fn mutate_with_marked(mut grid: Vec<Vec<char>>, marked: &Vec<[usize;2]>) -> Vec<Vec<char>> {
    for [i, j] in marked.iter() {
        match grid[*i][*j] {
            'L' => { grid[*i][*j] = '#' },
            '#' => { grid[*i][*j] = 'L' }
            _ => {}
        }
    }
    grid
}

pub fn count_occupied(grid: &Vec<Vec<char>>) -> u32 {
    grid.iter().fold(0, |acc, a| acc + a.iter().fold(0, |sum, c| sum + (*c == '#') as u32 ))
}