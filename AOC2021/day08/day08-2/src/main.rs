use std::fs;
// 8 => 7, 1 => 2, 4 => 4, 7 => 3
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines
        .split("\r\n")
        .map(|l| {
            l.split(" | ")
                .map(|part| part.split(' ').collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut ans = 0;
    for l in &lines {
        let mut possible_mappings = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];
        let mut actual_mappings = [""; 10];
        let mut four_for_five = "hiya!";
        for comb in &l[0] {
            match comb.len() {
                2 => {
                    match_combination_all(comb, &l[0], &mut possible_mappings, 0);
                    actual_mappings[1] = comb;
                }
                4 => {
                    match_combination_all(comb, &l[0], &mut possible_mappings, 3);
                    four_for_five = comb;
                    actual_mappings[4] = comb;
                }
                3 => {
                    match_combination_all(comb, &l[0], &mut possible_mappings, 6);
                    actual_mappings[7] = comb;
                }
                7 => {
                    actual_mappings[8] = comb;
                }
                _ => {}
            }
        }
        println!("{:?}", possible_mappings);
        possible_mappings[0] = possible_mappings[0]
            .clone()
            .into_iter()
            .filter(|p| !possible_mappings[3].contains(p))
            .collect::<Vec<_>>();
        actual_mappings[3] = possible_mappings[6][0];
        actual_mappings[9] = possible_mappings[3][0];
        actual_mappings[0] = possible_mappings[0][0];
        actual_mappings[6] = *l[0]
            .iter()
            .find(|s| {
                s.len() == 6 && **s != possible_mappings[3][0] && **s != possible_mappings[0][0]
            })
            .unwrap();
        actual_mappings[5] = l[0]
            .iter()
            .find(|s| {
                s.len() == 5
                    && four_for_five.chars().filter(|c| s.contains(*c)).count() == 3
                    && **s != actual_mappings[3]
            })
            .unwrap();
        let two = l[0].iter().find(|p| !actual_mappings.contains(p)).unwrap();
        actual_mappings[2] = *two;

        ans += translate_to_number(&l[1], &actual_mappings);
    }
    println!("{}", ans);
}
fn translate_to_number(ot: &[&str], mappings: &[&str]) -> i64 {
    ot.iter()
        .map(|f| {
            mappings
                .iter()
                .enumerate()
                .find(|(_, e)| {
                    let mut sorted_e = e.chars().collect::<Vec<_>>();
                    let mut sorted_f = f.chars().collect::<Vec<_>>();
                    sorted_e.sort_unstable();
                    sorted_f.sort_unstable();
                    sorted_e == sorted_f
                })
                .unwrap()
                .0
        })
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}
fn match_combination_all<'a, 'b>(
    pattern: &str,
    line: &'a [&'a str],
    possible_mappings: &'b mut [Vec<&'a str>],
    indx: usize,
) {
    match pattern.len() {
        2 | 4 => {
            possible_mappings[indx] = line
                .iter()
                .copied()
                .filter(|p| pattern.chars().all(|c| p.contains(c)) && p.len() == 6)
                .collect::<Vec<_>>();
        }
        3 => {
            possible_mappings[indx] = line
                .iter()
                .copied()
                .filter(|p| p.len() == 5 && pattern.chars().all(|c| p.contains(c)))
                .collect::<Vec<_>>();
        }
        _ => {}
    }
}
