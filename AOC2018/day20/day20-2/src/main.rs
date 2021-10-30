use std::{collections::{HashMap, HashSet, VecDeque}, fs};

fn main() {
    let pattern = fs::read_to_string("../input.txt").unwrap();
    
    let vectorified = &pattern[1..pattern.len()-1].chars().collect::<Vec<_>>();
    let mut up_to = Vec::new();
    let (paths, _) = map_builder(&vectorified, 0, &mut up_to, &find_groupings(&vectorified));
    
    // println!("{:?}", paths);
    let mut all_points = HashSet::new();

    for p in paths {
        let ret = path_to_point(&p);
        all_points.extend(ret);
    }   

    
    // let mut all_points: HashSet<(usize, usize)> = HashSet::new();

    // for p in paths.iter() {
    //     all_points.extend(path_to_point(p).iter());
    // }
    // // println!("{:?}", paths);
    let mut ans = 0;
    for targ in all_points.iter() {
        if shortest_path(&all_points, *targ) >= 1000 {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn shortest_path(paths: &HashSet<(usize, usize)>, goal: (usize, usize)) -> usize {
    // println!("{:?}", goal);
    let mut queue = VecDeque::new();

    let mut visited = HashSet::new();

    visited.insert((10000, 10000));
    queue.push_back((10000, 10000, 0));

    while !queue.is_empty() {

        let fr = queue.pop_front().unwrap();
        // println!("{:?} {:?}", fr, goal);

        for [y, x] in [[0, 1], [1, 0], [-1, 0], [0, -1]] {

            let (mut ny, mut nx) = (fr.0 as i64 + y, fr.1 as i64 + x);
            
            if ny >= 0 && nx >= 0 && paths.contains(&(ny as usize, nx as usize)) {
                if (ny as usize, nx as usize) == goal {
                    return fr.2 + 1;
                }
                
                if y == 1 { ny += 1 }
                if 1 == x { nx += 1 }
                if -1 == x { nx -= 1 }
                if -1 == y { ny -= 1 }

                if !visited.contains(&(ny, nx)) {
                    queue.push_back((ny as usize, nx as usize, fr.2 + 1));
                    visited.insert((ny, nx));
                }

            }


        }

    }
    // println!("{}", 1);
    0   
}

fn map_builder(pattern: &[char], mut index: usize, up_to_now: &mut Vec<String>, parens :&HashMap<usize, usize>) -> (Vec<String>, usize) {

    let mut ans = Vec::new();
    let mut temp = vec![String::new()];
    
    while index < pattern.len()
    {
        let c = pattern[index];
        match  c {
            '|' => {
                ans.extend(temp.clone().into_iter());
                temp = vec![String::new()];
            }

            'S' | 'N' | 'E' | 'W'  => {
                temp[0].push(pattern[index]);
            }

            '(' => {
                let (vecs, indx) = map_builder(pattern, index + 1, up_to_now, parens);
                let mut tmp = Vec::new();

                for tm in vecs {
                    tmp.push(temp[0].clone() + &tm);
                }
                temp = tmp;
                index = indx;
            }
            ')' => {
                ans.extend(temp.into_iter());
                return (ans, index);
            }
            _ => {}
        }
        index += 1;
    }
    ans.extend(temp.into_iter());
    (ans, index)

}

fn path_to_point(path: &str) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut p = (10000, 10000);
    for c in path.chars() {
        match c {
            'N' => {   
                p.0 -= 1;
                visited.insert(p);
                p.0 -= 1;
            }
            'E' => {
                p.1 += 1;
                visited.insert(p);
                p.1 += 1;
            }
            'W' => {
                p.1 -= 1;
                visited.insert(p);
                p.1 -= 1;
            }
            'S' => {
                p.0 += 1;
                visited.insert(p);
                p.0 += 1;
            }
            _ => {}
        }
        
    }
    visited
}


fn find_groupings(pattern: &[char]) -> HashMap<usize, usize> {
    // println!("{:?}", pattern);
    let mut stack: Vec<usize> = Vec::with_capacity(30);
    let mut ans = HashMap::with_capacity(10);

    for (i, c) in pattern.iter().enumerate() {
        if *c == '(' {
            stack.push(i);
        } else if *c == ')' {
            ans.insert(stack.pop().unwrap(), i);
        }
    }
    // println!("{:?}", ans);
    ans
}