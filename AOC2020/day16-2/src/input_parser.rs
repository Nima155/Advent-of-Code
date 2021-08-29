use std::collections::{HashMap, HashSet};
pub fn vec_if_valid(line: &str, valids: &[bool]) -> Option<Vec<usize>> {
    let nums = line
        .split(',')
        .map(|e| e.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    if nums.iter().any(|z| !valids[*z]) {
        return None;
    }
    Some(nums)
}

pub fn recursive_field_assignment<'a>(
    rows: &'a [Vec<usize>],
    fields: &'a HashMap<&'a str, HashSet<usize>>,
    map: &mut HashMap<usize, HashSet<&'a str>>,
    seen: &mut HashSet<&'a str>,
) {
    for (k, v) in fields {
        if !seen.contains(*k) {
            for i in 0..rows[0].len() {
                if rows.iter().all(|s| v.contains(&s[i])) {
                    match map.get_mut(&i) {
                        Some(z) => {
                            z.insert(*k);
                        }
                        None => {
                            map.insert(i, HashSet::new());
                            map.get_mut(&i).unwrap().insert(*k);
                        }
                    }
                }
            }
        }
    }

    let mut to_rem = "";
    if let Some(thing) = map
        .iter_mut()
        .find(|s| s.1.len() == 1 && !seen.contains(s.1.iter().next().unwrap()))
    {
        to_rem = *thing.1.iter().next().unwrap();
    }

    if !to_rem.is_empty() {
        for (_, v) in map.iter_mut() {
            if v.contains(to_rem) && v.len() > 1 {
                v.remove(to_rem);
            }
        }
        seen.insert(to_rem);

        recursive_field_assignment(rows, fields, map, seen)
    }
}
