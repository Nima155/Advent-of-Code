// use std::vec;
// target area: x=20..30, y=-10..-5
// target area: x=153..199, y=-114..-75
const X_MIN: i64 = 153;
const X_MAX: i64 = 199;

const Y_MIN: i64 = -114;
const Y_MAX: i64 = -75;
fn main() {
    let mut y_max = i64::MIN;
    let mut ans = 0;
    for i in 1..200 {
        for j in -114..1000 {
            y_max = i64::max(y_max, highest_y_reached((i, j)));
            if y_max != i64::MIN {
                y_max = i64::MIN;
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn highest_y_reached(mut velocity: (i64, i64)) -> i64 {
    let (mut x, mut y) = (0, 0);

    let mut max_y = i64::MIN;
    
    while x <= X_MAX && y >= Y_MIN {
        max_y = i64::max(max_y, y);
        if X_MIN <= x && x <= X_MAX && Y_MIN <= y && y <= Y_MAX {
            return max_y;
        }
        y += velocity.1;

        x += velocity.0;
        velocity.0 = if velocity.0 < 0 {
            velocity.0 + 1
        } else if velocity.0 > 0 {
            velocity.0 - 1
        } else {
            0
        };

        velocity.1 -= 1;
    }
    i64::MIN
}
