use std::{collections::HashMap, fs};
fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let mapped = parse(&read);

    
    
    let mut mappy = HashMap::new();
    // part 2 
    println!("{}", backward_dfs(&mapped, "FUEL", 1, &mut mappy) as i64);
    println!("{}", part_2(&mapped))
}
// start from fuel and build up from there ?
fn part_2(mapped: &HashMap<&str, (u64, Vec<(u64, &str)>)>) -> u64 {
    let cap = 1_000_000_000_000i64;
    let (mut end, mut start) = (10000000, 1);

    while start <= end {
        let middle = (end + start) / 2;
        let anso = backward_dfs(mapped, "FUEL", middle as u64, &mut HashMap::new()) as i64;
        
        let val = cap - anso;
        if val < 0 {
            end = middle - 1;
        } 
        else if val > 0 {
            start = middle + 1;
        }
        else {
            return middle as u64;
        }

    }
    
    end as u64
}
fn parse(read: &str) -> HashMap<&str, (u64, Vec<(u64, &str)>)> {
    read.split("\r\n")
        .map(|l| {
            let chemicals = l.split("=>").collect::<Vec<_>>();
            let chemical_destination = chemicals[1].trim().split(' ').collect::<Vec<_>>();
            let chemical_recipe = chemicals[0]
                .trim()
                .split(", ")
                .map(|e| {
                    let to_veced = e.trim().split(' ').collect::<Vec<_>>();
                    (to_veced[0].parse::<u64>().unwrap(), to_veced[1])
                })
                .collect::<Vec<_>>();

            (
                chemical_destination[1],
                (
                    chemical_destination[0].parse::<u64>().unwrap(),
                    chemical_recipe,
                ),
            )
        })
        .collect::<HashMap<_, _>>()
}

fn backward_dfs<'a>(
    chemicals: &'a HashMap<&str, (u64, Vec<(u64, &str)>)>,
    cur_chem: &'a str,
    mut req_am: u64,
    rems: &mut HashMap<&'a str, u64>,
) -> u64 // cost amount
{
    let mut ans = 0;

    if rems.contains_key(cur_chem) {
        let get_em = rems.get_mut(cur_chem).unwrap();
        
        match *get_em >= req_am  {
            true => { *get_em -= req_am; return 0; },
            false => { req_am -= *get_em; *get_em = 0; },
        }
    }


    let will_be_given = chemicals.get(cur_chem).unwrap().0;

    let tots = (req_am as f64 / will_be_given as f64).ceil() as u64;

    let rem = (tots * will_be_given) - req_am;
    
    
    if rem != 0 {
        match rems.get_mut(cur_chem) {
            Some(z) => {
                *z += rem;
            }
            None => {
                rems.insert(cur_chem, rem);
            }
        }
    }
    
    for (d, ot) in &chemicals.get(cur_chem).unwrap().1 {
        if *ot == "ORE" {
            return *d * tots;
        }
        let zz = backward_dfs(chemicals, *ot, *d * tots , rems);
        
        ans += zz;
    }

    ans
}
