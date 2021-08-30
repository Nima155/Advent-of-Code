use dimensional_ops::{get_active_nodes, node_cycle};

mod dimensional_ops;

fn main() {
    let input = "#......#\n##.#..#.\n#.#.###.\n.##.....\n.##.#...\n##.#....\n#####.#.\n##.#.###"
        .split('\n');

    let _mock_input = ".#.\n..#\n###".split('\n');

    let grid = input
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut actives = get_active_nodes(&grid);

    let mut cycles = 0;

    while cycles < 6 {
        actives = node_cycle(&actives);
        cycles += 1;
    }
    println!("{}", actives.len());
}
