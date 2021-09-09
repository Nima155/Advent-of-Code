use indexmap::IndexMap;

pub fn find_new_dest(mut cur: i32, picked: &[i32]) -> i32 {
    loop {
        cur -= 1;
        if cur == 0 {
            cur = 1000000;
        }
        if !picked.contains(&cur) {
            return cur;
        }
    }
}

pub fn play_the_game(circle: &mut IndexMap<i32, i32>) {
    let (mut first, mut cycle) = (*circle.first().unwrap().0, 0);

    while cycle < 10000000 {
        let (mut picks, mut cur_node, mut cur_indx): ([i32; 3], i32, usize) =
            ([0; 3], *circle.get(&first).unwrap(), 0);

        while cur_indx < 3 {
            picks[cur_indx] = cur_node;
            cur_node = *circle.get(&cur_node).unwrap();
            cur_indx += 1;
        }

        let destination = find_new_dest(first, &picks);
        let destination_neighbor = *circle.get(&destination).unwrap();

        circle
            .entry(*picks.last().unwrap())
            .and_modify(|f| *f = destination_neighbor); // last neighbors new neighbor

        circle.entry(first).and_modify(|f| *f = cur_node);

        circle
            .entry(destination)
            .and_modify(|f| *f = *picks.first().unwrap());

        first = cur_node;

        cycle += 1;
    }
}
