use std::collections::{HashMap, HashSet};

pub fn map_the_map<'a>(
    mappings: &HashMap<usize, (HashSet<&'a str>, HashSet<&'a str>)>,
) -> HashMap<&'a str, HashMap<&'a str, u32>> {
    let mut mapped = HashMap::new();

    for (ings, allergens) in mappings.values() {
        for allergen in allergens {
            for ing in ings {
                if !ing.is_empty() {
                    let count = mapped
                        .entry(*allergen)
                        .or_insert_with(HashMap::new)
                        .entry(*ing)
                        .or_insert(0);
                    *count += 1;
                }
            }
        }
    }
    mapped
}

pub fn find_the_actual_mappings<'a>(
    mut mappings: HashMap<&'a str, HashSet<&'a str>>,
) -> (
    HashMap<&'a str, &'a str>,
    HashMap<&'a str, HashSet<&'a str>>,
) {
    let mut ans = HashMap::new();

    for (allerg, mappi) in mappings.clone() {
        if mappi.len() == 1 {
            let ele = *mappi.iter().next().unwrap();
            ans.insert(allerg, ele);

            for mappis in mappings.values_mut() {
                mappis.remove(ele);
            }

            let (ans_sub, mappingsz) = find_the_actual_mappings(mappings);

            for (allrg, name) in ans_sub {
                ans.insert(allrg, name);
            }
            mappings = mappingsz;
        }
    }

    (ans, mappings)
}

pub fn process_the_map<'a>(
    allerg_to_ings: HashMap<&'a str, HashMap<&'a str, u32>>,
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut possible_allergen_mappings = HashMap::new();

    for (allerg, counts) in allerg_to_ings {
        let mut cur_max = 0;
        let mut temp = HashSet::new();
        for (name, count) in counts {
            if count > cur_max {
                cur_max = count;
                temp.clear();
                temp.insert(name);
            } else if count == cur_max {
                temp.insert(name);
            }
        }
        possible_allergen_mappings.insert(allerg, temp);
    }
    possible_allergen_mappings
}
