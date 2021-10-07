use std::fs;

// Main idea comes from code forces.. I would not have been able to solve this without the help of other people.
// https://codeforces.com/blog/entry/72593
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let actions = lines
        .split("\r\n")
        .map(|l| match (l.ends_with("stack"), l.starts_with("cut")) {
            (true, _) => "stack".to_owned(),
            (_, true) => l.to_owned(),
            (false, false) => {
                format!("inc {}", l.split(' ').last().unwrap())
            }
        })
        .collect::<Vec<_>>();

    // println!("{}", (-11 % 2) + 2);

    println!("{}", play_cards(&actions));
}
const M: i128 = 119315717514047_i128;
#[derive(Debug)]
struct Lcf {
    a: i128,
    b: i128,
}

impl Lcf {
    fn compose(&self, (a, b): (i128, i128)) -> Self {
        // println!("{} {} {} {}", self.a, a, m, self.b);
        // let m = if  (self.a * a) % m == 0 { m - 1 } else { m };
        Self {
            a: (((self.a * a) % M) + M) % M,
            b: (((b + (a * self.b)) % M) + M) % M,
        }
    }
}

impl Default for Lcf {
    fn default() -> Self {
        Self { a: 1, b: 0 }
    }
}

fn play_cards(actions: &[String]) -> i128 {
    // input is prime.. and sequence is repeated on prime!
    let mut initial: Lcf = Lcf::default();
    for act in actions {
        match (act.contains("stack"), act.contains("inc")) {
            (true, _) => {
                initial = initial.compose((-1, -1));
            }
            (_, true) => {
                let num = act.split_at(3).1.trim().parse::<i128>().unwrap();
                initial = initial.compose((num, 0));
            }
            (_, _) => {
                let num = act.split_at(3).1.trim().parse::<i128>().unwrap();
                initial = initial.compose((1, -num));
            }
        }
        // println!("{}  {}", initial.a, initial.b);
    }
    let resultant_lcf = pow_compose(initial, 101741582076661);

    ((((2020 - resultant_lcf.b) * pow_mod(resultant_lcf.a, M - 2)) % M) + M) % M // 
}

fn pow_compose(mut f: Lcf, mut k: i128) -> Lcf {
    let mut g = Lcf { a: 1, b: 0 };
    while k > 0 {
        if k % 2 != 0 {
            g = g.compose((f.a, f.b));
        }
        k /= 2;
        f = f.compose((f.a, f.b));
    }
    g
}
// repeated code.. but hey...
fn pow_mod(mut f: i128, mut k: i128) -> i128 {
    let mut g: i128 = 1;

    while k > 0 {
        if k % 2 != 0 {
            g = (g * f) % M
        }
        k /= 2;
        f = (f * f) % M;
    }
    g
}
