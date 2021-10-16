use std::fs;
fn main() {
    let tree = fs::read_to_string("../input.txt").unwrap();

    let tree_info = tree
        .split(' ')
        .map(|f| f.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    // println!("{:?}", tree_info);
    println!("{:?}", find_meta(&tree_info, 0).0);
}
// go up in increments of two!
fn find_meta(tree_info: &Vec<i64>, mut cur_indx: usize) -> (i64, usize) {
    // println!("{}", cur_indx);
    if tree_info[cur_indx] == 0 {
        // println!("{:?}", &tree_info[(cur_indx + 2) as usize..cur_indx + 2 + (tree_info[cur_indx + 1]) as usize]);
        return (
            *&tree_info[(cur_indx + 2) as usize..cur_indx + 2 + (tree_info[cur_indx + 1]) as usize]
                .iter()
                .sum::<i64>(),
            cur_indx + tree_info[cur_indx + 1] as usize + 2,
        );
    }

    let (mut ans, indx) = (0, cur_indx);

    for _ in 0..tree_info[cur_indx] {
        let ret = find_meta(tree_info, if indx != cur_indx { cur_indx } else { cur_indx + 2 });
        ans += ret.0;    
        cur_indx = ret.1;
    }

    (
        ans + tree_info[cur_indx..cur_indx + tree_info[indx + 1] as usize]
            .iter()
            .sum::<i64>(),
        cur_indx + tree_info[indx + 1] as usize,
    )
}

