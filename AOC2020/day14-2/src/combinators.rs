// use core::num;
use std::collections::HashMap;

pub fn all_combinations(
    mask: &[(usize, u32)],
    number: u64,
    memory: &mut HashMap<u64, u64>,
    indx: usize,
    mut key: u64,
) {
    if indx >= mask.len() {
        memory.insert(key, number);
        return;
    }

    let mut other = key;

    key |= 1u64 << (mask[indx].0 as u64);

    if let (_, 2) = mask[indx] {
        other &= !(1u64 << mask[indx].0 as u64);

        all_combinations(mask, number, memory, indx + 1, other)
    }
    all_combinations(mask, number, memory, indx + 1, key)
}
