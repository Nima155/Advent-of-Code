// 300 * 300
fn main() {
    // 9445
    let mut grid = [[0; 300]; 300];

    let mut temp = i64::MIN;
    traverse_and_assign(&mut grid);
    for i in 1..301 {
        let inter_med = best_square(&grid, i);
        if inter_med.1 > temp {
            temp = inter_med.1;
            println!("{:?} {}", inter_med, i); // last one printed is the answer!
        }
    }
    
}   

fn traverse_and_assign(grid: &mut [[i64; 300]; 300]) {

    for i in 0..300 {
        for j in 0..300 {
            grid[i][j] = calculate_power((i + 1, j + 1));
        }
    }
}

fn best_square(grid: &[[i64; 300]; 300], square_size: usize) -> ((usize, usize), i64) {
    let mut ans = i64::MIN;
    let mut actual_ans = (0, 0);
    'a: for i in 0..300 {
        for j in 0..300 {

            let j_o = j + square_size;
            let i_o = i + square_size;
            if j_o > 300 {
                break;
            }
            if i_o > 300 {
                break 'a;
            }
            let sums = grid[i.. i_o].iter().fold(0, |acc,r| r[j..j_o].iter().sum::<i64>() + acc);
            if sums > ans {
                ans = sums;
                actual_ans = (j + 1, i + 1);
            }
            
        }
    }
    // println!("{}", ans);
    (actual_ans, ans)
}


const SERIAL_NUMBER: i64 = 9445;

fn calculate_power((y, x): (usize, usize)) -> i64 {
    let rack_id = x as i64 + 10;
    let y = y as i64;
    ((((rack_id * y) + SERIAL_NUMBER) * rack_id) / 100) % 10 - 5

}
