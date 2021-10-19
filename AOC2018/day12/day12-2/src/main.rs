use std::{collections::BTreeMap, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let state_and_instrcutions = lines.split("\r\n\r\n").collect::<Vec<_>>();

    let state = state_and_instrcutions[0].split(": ").collect::<Vec<_>>()[1];
    let state = state
        .chars()
        .enumerate()
        .map(|f| (f.0 as i64 - 2, f.1))
        .collect::<BTreeMap<i64, _>>();

    let instructions = state_and_instrcutions[1]
        .split("\r\n")
        .map(|l| l.split(" => ").collect::<Vec<_>>())
        .collect::<Vec<_>>();


    // this will eventually print numbers that go up by 50 each time!
    /* 
        for i in 20..2000 {
            println!("{} {}", run_cycle(&state, &instructions, i), i);
        } 
    */

    // so
    println!("{}", (50_000_000_000_i128 - 100) * 50 + 6175); // this gives us our answer!
    
}

fn run_cycle(state: &BTreeMap<i64, char>, instructions: &[Vec<&str>], mut cycles: usize) -> i64 {
    let mut state = state.clone();
    // 2672 false!
    while cycles > 0 {
        let mut ot = BTreeMap::new();
        // println!("{:?}\n\n", state);
        for (i, e) in state.clone().iter() {
            for ins in instructions {
                let (i_bef_2, i_bef_1, i_aft_1, i_aft_2) = (i - 2, i - 1, i + 1, i + 2);
                let (bef_2_node, bef_1_node, aft_1_node, aft_2_node) = (
                    *state.entry(i_bef_2).or_insert('.'),
                    *state.entry(i_bef_1).or_insert('.'),
                    *state.entry(i_aft_1).or_insert('.'),
                    *state.entry(i_aft_2).or_insert('.'),
                );

                if [bef_2_node, bef_1_node, *e, aft_1_node, aft_2_node]
                    == [
                        ins[0].chars().next().unwrap(),
                        ins[0].chars().nth(1).unwrap(),
                        ins[0].chars().nth(2).unwrap(),
                        ins[0].chars().nth(3).unwrap(),
                        ins[0].chars().nth(4).unwrap(),
                    ]
                {
                    ot.insert(*i, ins[1].chars().next().unwrap());
                    break;
                }
            }
        }
        for st in *state.iter().min().unwrap().0 - 2..*state.iter().max().unwrap().0 + 2 {
            ot.entry(st).or_insert('.');
        }
        state = ot;
        // println!("{}", state.iter().filter(|e| *e.1 == '#').fold(0, |acc, cur| acc + *cur.0));

        cycles -= 1;
    }
    // println!("{:?}", state);
    state
        .iter()
        .filter(|e| *e.1 == '#')
        .fold(0, |acc, cur| acc + *cur.0)
    // let vec![;]
}
