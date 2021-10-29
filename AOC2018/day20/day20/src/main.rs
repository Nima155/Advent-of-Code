use std::{collections::{HashMap, HashSet, VecDeque}, fs};

fn main() {
    let pattern = fs::read_to_string("../input.txt").unwrap();
    
    let vectorified = &pattern[1..pattern.len()-1].chars().collect::<Vec<_>>();
    
    let paths = map_builder(&vectorified, 0, &find_groupings(&vectorified));
    // println!("{:?}", paths);
    // println!("{:?}", paths);
    let mut all_points: HashSet<(usize, usize)> = HashSet::new();

    for p in paths.iter() {
        all_points.extend(path_to_point(p).iter());
    }
    // println!("{:?}", paths);
    let mut ans = 0;
    for targ in all_points.iter() {
        
        ans = usize::max(ans,shortest_path(&all_points, *targ)) ;
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

fn map_builder(pattern: &[char], index: usize, paren_mappings: &HashMap<usize, usize>) -> Vec<String> {
    let mut ans = vec![String::new()];
    let (mut i, mut mark) = (index, -1);
    let mut or_mode = false;
    
    if index != 0 && pattern[index - 1] == '(' {
        or_mode = true;
    } 

    while i < pattern.len() {
        let c = pattern[i];
        match c {
            'N' | 'E' | 'S' | 'W' => {
                if !or_mode {
                    for s in ans.iter_mut() {
                        if !or_mode || s.starts_with(&pattern[index..mark as usize]){

                            s.push(c);
                        }
                    }
                } else {
                    if mark != - 1 {
                        
                        
                        for s in ans.iter_mut() {
                            if s.starts_with(&pattern[index..mark as usize]) {
                                s.push(c);
                            }
                        }
                        
                    } else {
                        
                        ans.last_mut().unwrap().push(c);
                    }
                }
            }
            '(' => {
                mark = i as i64;
                let builds = map_builder(pattern, i + 1, paren_mappings);
                let mut temp_ans = vec![];
                if !or_mode {
                    for g in ans.iter() {
                        for gr in builds.iter() {            
                            temp_ans.push(g.clone() + gr);
                        }
                    }
                } else {
                    temp_ans = ans[..ans.len() - 1].to_owned();
                    for gr in builds.iter() {
                        temp_ans.push(ans.last().unwrap().clone() + gr);
                    }
                }
                i = *paren_mappings.get_key_value(&i).unwrap().1;
                ans = temp_ans;
                // println!("{:?} {:?}", ans, i);
            }
            ')' => {
                return ans;
            }
            '|' => {
                mark = -1;
                ans.push(String::new());
            }
            _ => {}
        }
        i += 1;
    }
    ans
    
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