// Player 1 starting position: 4
// Player 2 starting position: 5
fn main() {
    println!("{}", first_loser(4, 5));
}

fn next_toss(mut cur_side: i64) -> i64 {
    let mut cc = 0;
    let mut ans = 0;

    while cc < 3 {
        let side = cur_side % 100;

        ans += if side == 0 { 100 } else { side };
        cur_side += 1;
        cc += 1;
    }

    ans
}

fn first_loser(mut p1_pos: i64, mut p2_pos: i64) -> i64 {
    let [mut p1, mut p2] = [0, 0];

    let [mut turn, mut current_dice] = [0, 1];
    let mut tosses = 0;
    while p1 < 1000 && p2 < 1000 {
        if turn == 0 {
            let mut next_pos = next_toss(current_dice) + p1_pos;

            if next_pos % 10 == 0 && next_pos > 10 {
                next_pos = 10;
            } else if next_pos > 10 {
                next_pos %= 10;
            }

            p1_pos = next_pos;

            p1 += p1_pos;
        } else {
            let mut next_pos = next_toss(current_dice) + p2_pos;

            if next_pos % 10 == 0 && next_pos > 10 {
                next_pos = 10;
            } else if next_pos > 10 {
                next_pos %= 10;
            }

            p2_pos = next_pos;
            // break;
            p2 += p2_pos;
        }
        current_dice = if current_dice + 3 > 100 {
            (current_dice + 3) % 100
        } else {
            current_dice + 3
        };

        tosses += 3;
        turn ^= 1;
        // println!("{} {}", p1, p2);
    }

    tosses * i64::min(p1, p2)
}
