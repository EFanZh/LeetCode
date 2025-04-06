pub mod topological_sorting;

pub trait Solution {
    fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;
    use std::iter;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    8,
                    2,
                    &[-1, -1, 1, 0, 0, 1, 0, -1] as &[_],
                    &[&[] as &[_], &[6], &[5], &[6], &[3, 6], &[], &[], &[]] as &[&[_]],
                ),
                true,
            ),
            (
                (
                    8,
                    2,
                    &[-1, -1, 1, 0, 0, 1, 0, -1],
                    &[&[], &[6], &[5], &[6], &[3], &[], &[4], &[]],
                ),
                false,
            ),
            (
                (
                    10,
                    4,
                    &[0, 1, 1, 2, 3, -1, 0, 0, 0, 1],
                    &[&[2, 5], &[3, 5, 4, 6, 8, 7, 2], &[7], &[], &[], &[], &[], &[], &[], &[]],
                ),
                false,
            ),
        ];

        let mut item_positions = Vec::new();
        let mut group_stats = Vec::<(usize, usize, usize)>::new();

        for ((n, m, group, before_items), has_solution) in test_cases {
            let result = S::sort_items(
                n,
                m,
                group.to_vec(),
                before_items.iter().copied().map(Vec::from).collect(),
            );

            if has_solution {
                // Check elements.

                assert!((0..n).eq(test_utilities::unstable_sorted(result.iter().copied())));

                // Check grouping.

                item_positions.resize(n as _, 0);
                group_stats.extend(iter::repeat_n((usize::MAX, 0, 0), m as _));

                for (position, &item) in result.iter().enumerate() {
                    let item = item as usize;

                    item_positions[item] = position;

                    if let Some((first, last, count)) = group_stats.get_mut(group[item] as usize) {
                        if *first == usize::MAX {
                            *first = position;
                        }

                        *last = position;
                        *count += 1;
                    }
                }

                for &(first, last, count) in &group_stats {
                    assert_eq!(last + 1 - first, count);
                }

                // Check dependencies.

                for (item, &before) in before_items.iter().enumerate() {
                    let item_position = item_positions[item];

                    for &before_item in before {
                        assert!(item_positions[before_item as usize] < item_position);
                    }
                }

                item_positions.clear();
                group_stats.clear();
            } else {
                assert!(result.is_empty());
            }
        }
    }
}
