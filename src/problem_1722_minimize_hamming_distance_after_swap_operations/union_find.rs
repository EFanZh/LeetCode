pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    fn find_root_helper(union_find: &mut [(u32, u32)], node: u32) -> Option<(u32, u32)> {
        let node = node as usize;

        union_find.get(node).copied().map(|(parent_plus_1, rank)| {
            let root = if let Some((root, _)) = Self::find_root_helper(union_find, parent_plus_1.wrapping_sub(1)) {
                union_find[node].0 = root + 1;

                root
            } else {
                node as _
            };

            (root, rank)
        })
    }

    fn find_root(union_find: &mut [(u32, u32)], node: u32) -> (u32, u32) {
        Self::find_root_helper(union_find, node).unwrap()
    }

    fn union(union_find: &mut [(u32, u32)], x: u32, y: u32) {
        let left = Self::find_root(union_find, x);
        let right = Self::find_root(union_find, y);

        if left.0 != right.0 {
            let (child, parent) = if left.1 < right.1 {
                (left.0, right.0)
            } else {
                if left.1 == right.1 {
                    union_find[left.0 as usize].1 += 1;
                }

                (right.0, left.0)
            };

            union_find[child as usize].0 = parent + 1;
        }
    }

    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        let n = source.len();
        let mut union_find = vec![(0, 0); n].into_boxed_slice();

        for allowed_swap in allowed_swaps {
            let [x, y] = allowed_swap.try_into().ok().unwrap();

            Self::union(&mut union_find, x as _, y as _);
        }

        let mut groups = vec![HashMap::<i32, i32>::new(); n];

        for (i, (top, bottom)) in (0..).zip(source.into_iter().zip(target)) {
            if top != bottom {
                let diffs = &mut groups[Self::find_root(&mut union_find, i).0 as usize];

                for (value, diff) in [(top, 1), (bottom, -1)] {
                    match diffs.entry(value) {
                        Entry::Occupied(entry) => *entry.into_mut() += diff,
                        Entry::Vacant(entry) => drop(entry.insert(diff)),
                    }
                }
            }
        }

        let mut result = 0;

        for group in groups {
            for diff in group.into_values() {
                if diff > 0 {
                    result += diff;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        Self::minimum_hamming_distance(source, target, allowed_swaps)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
