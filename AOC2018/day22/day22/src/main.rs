// depth: 7305
// target: 13,734
// The region at 0,0 (the mouth of the cave) has a geologic index of 0.
// The region at the coordinates of the target has a geologic index of 0.
// If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807.
// If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271.
// Otherwise, the region's geologic index is the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1.

// A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183. Then: % 3
fn main() {
    println!("{}", calculate_erosion_levels(734, 13, 7305));
}

const MODULO_CONST: i128 = 20183;
const TYPE_MODULO: i128 = 3;

fn calculate_erosion_levels(height: usize, width: usize, depth: usize) -> i128 {
    let mut ans = vec![vec![0i128; width + 1]; height + 1];
    let mut real_ans = 0;
    // 10378 too high
    for i in 0..=height {
        for j in 0..=width {
            let mut geo_indx = if i == 0 {
                j as i128 * 16807
            } else if j == 0 {
                i as i128 * 48271
            } else {
                ans[i][j - 1] * ans[i - 1][j]
            };

            if i == height && j == width {
                geo_indx = 0;
            }

            // println!("{}", geo_indx);

            ans[i][j] = (geo_indx + depth as i128) % MODULO_CONST;

            real_ans += ans[i][j] % TYPE_MODULO;
            // print!("{}", if ans[i][j] % TYPE_MODULO == 2  { '|' } else if ans[i][j] % TYPE_MODULO == 1  { '=' } else { '.' });
        }
    }

    real_ans
}
