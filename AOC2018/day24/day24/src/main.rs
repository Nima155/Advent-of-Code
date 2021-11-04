use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::Reverse, collections::HashSet, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let mut battlefield_durum_durum = vec![];

    if let [immuz, infecc] = lines.split("\r\n\r\n").collect::<Vec<_>>()[..] {
        battlefield_durum_durum = immuz
            .split("\r\n")
            .skip(1)
            .map(line_parser)
            .collect::<Vec<_>>();
        battlefield_durum_durum.extend(infecc.split("\r\n").skip(1).map(|l| {
            let mut res = line_parser(l);
            res.5 = true;
            res
        }))
    }
    println!("{}", play_the_game(battlefield_durum_durum));
}
//  23376 too low
type Stats<'a> = (
    i64,
    i64,
    Option<Vec<(&'a str, &'a str)>>,
    (i64, &'a str),
    i64,
    bool,
);

fn line_parser<'a>(line: &'a str) -> Stats {
    lazy_static! {
        static ref RE: Regex = Regex::new(r###"(?P<units>\d+).+?(?P<hits>\d+).+?(?P<others>[(].*?[)]|with).+?(?P<attack>\d+\s[a-z]+).+?(?P<initiative>\d+)"###).unwrap();
    }

    if let Some(cap) = RE.captures_iter(line).next() {
        let others = cap.name("others").unwrap().as_str();
        let mut parsed = None;
        if !others.starts_with("with") {
            parsed = Some(
                others[1..others.len() - 1]
                    .split("; ")
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|s| s.split(' ').collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            );
        }

        let mut others_parsed = None;

        if let Some(v) = parsed {
            others_parsed = Some(
                v.iter()
                    .map(|v| v[2..].iter().map(|t| (v[0], *t)).collect::<Vec<_>>())
                    .flatten()
                    .collect::<Vec<_>>(),
            );
        }

        let attacks = cap
            .name("attack")
            .unwrap()
            .as_str()
            .split(' ')
            .collect::<Vec<_>>();

        return (
            cap.name("units").unwrap().as_str().parse::<i64>().unwrap(),
            cap.name("hits").unwrap().as_str().parse::<i64>().unwrap(),
            others_parsed,
            (attacks[0].parse::<i64>().unwrap(), attacks[1]),
            cap.name("initiative")
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap(),
            false,
        );
    }

    (0, 0, None, (10, ""), 10, false)
}

// units, hits, immunities and weaknesses, attack, initiative
fn play_the_game(mut battlefield_durum_durum: Vec<Stats>) -> i64 {
    loop {
        // println!("{}", battlefield_durum_durum);
        let mut moved_combatants = HashSet::new();
        let mut targeted_combatants: HashSet<i64> = HashSet::new();
        let length = battlefield_durum_durum.len();
        let mut attacking_phase = vec![];
        for _ in 0..length {
            let combatant = battlefield_durum_durum
                .iter()
                .filter(|c| !moved_combatants.contains(&c.1))
                .max_by(|a, b| (a.0 * a.3 .0, a.4).cmp(&(b.0 * b.3 .0, b.4)));
            if combatant.is_some() {
                let combatant = combatant.unwrap().clone();
                moved_combatants.insert(combatant.1);
                if let Some(defender) = find_opponent(
                    &battlefield_durum_durum,
                    &combatant,
                    &mut targeted_combatants,
                ) {
                    let defender = battlefield_durum_durum
                        .iter()
                        .find(|e| e.1 == defender)
                        .unwrap()
                        .clone();

                    // println!("{:?}", (&defender, combatant));
                    // each unit has this hp!
                    attacking_phase.push((defender.1, combatant.1, combatant.4));
                }
            }
        }

        attacking_phase.sort_by_key(|k| Reverse(k.2));
        for (def, comb, _) in attacking_phase {
            let comb = battlefield_durum_durum
                .iter()
                .find(|e| e.1 == comb)
                .unwrap()
                .clone();
            let defender = battlefield_durum_durum
                .iter_mut()
                .find(|e| e.1 == def)
                .unwrap();

            let dmg = check_combat_compatibility(&comb, defender);
            // println!("{} {:?}",  dmg, defender);
            let dmg = dmg - (dmg % defender.1);
            let defo_0 = defender.0 - (dmg / defender.1);
            if defo_0 > 0 {
                defender.0 = defo_0 as i64;
            } else {
                defender.0 = 0;
                moved_combatants.insert(defender.1);
            }
            // println!("{}", defender.0);
        }

        battlefield_durum_durum = battlefield_durum_durum
            .into_iter()
            .filter(|d| d.0 > 0)
            .collect::<Vec<_>>();
        let gr = battlefield_durum_durum[0].5;
        if battlefield_durum_durum.iter().all(|s| s.5 == gr) {
            return battlefield_durum_durum.iter().map(|s| s.0).sum();
        }
    }
}

fn find_opponent(
    battlefield_durum_durum: &[Stats],
    aggressor: &Stats,
    targeted_combatants: &mut HashSet<i64>,
) -> Option<i64> {
    let mut opponents: Vec<&Stats> = battlefield_durum_durum
        .iter()
        .filter(|c| c.5 != aggressor.5)
        .collect::<Vec<_>>();

    opponents.sort_by(|a, b| {
        (check_combat_compatibility(aggressor, b), b.0 * b.3 .0, b.4).cmp(&(
            check_combat_compatibility(aggressor, a),
            a.0 * a.3 .0,
            a.4,
        ))
    });

    for op in opponents {
        if !targeted_combatants.contains(&op.1) {
            targeted_combatants.insert(op.1);
            return Some(op.1);
        }
    }
    None
}

fn check_combat_compatibility(aggressor: &Stats, defender: &Stats) -> i64 {
    let (dmg, aggressor_attack_type) = aggressor.3;

    // aggressor attack vs defender...
    for (type_, kind) in &defender.2.clone().or_else(|| Some(vec![])).unwrap() {
        let n_kind = kind.trim_matches(',');
        match *type_ {
            "weak" => {
                if aggressor_attack_type == n_kind {
                    return (dmg * aggressor.0) * 2;
                }
            }
            "immune" => {
                if aggressor_attack_type == n_kind {
                    return 0;
                }
            }
            _ => {}
        }
    }
    dmg * aggressor.0
}
