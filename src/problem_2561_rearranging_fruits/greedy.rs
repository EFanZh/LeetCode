pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter;

impl Solution {
    fn count(values: &[u32]) -> HashMap<u32, u32> {
        let mut result = HashMap::new();

        for &value in values {
            match result.entry(value) {
                Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            }
        }

        result
    }

    fn compare(
        lhs: HashMap<u32, u32>,
        mut rhs: HashMap<u32, u32>,
        mut unique: impl FnMut(u32),
        mut xor: impl FnMut(u32, u32) -> Option<()>,
    ) -> Option<()> {
        for (key, left_count) in lhs {
            unique(key);

            xor(
                key,
                if let Some(right_count) = rhs.remove(&key) {
                    match left_count.cmp(&right_count) {
                        Ordering::Less => right_count - left_count,
                        Ordering::Equal => continue,
                        Ordering::Greater => left_count - right_count,
                    }
                } else {
                    left_count
                },
            )?;
        }

        for (key, count) in rhs {
            unique(key);
            xor(key, count)?;
        }

        Some(())
    }

    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let basket1 = basket1.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let basket2 = basket2.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let count_1 = Self::count(&basket1);
        let count_2 = Self::count(&basket2);

        drop(basket2);

        let mut min = u32::MAX;
        let mut buffer = basket1;

        buffer.clear();

        if Self::compare(
            count_1,
            count_2,
            |value| min = min.min(value),
            |value, count| (count % 2 == 0).then(|| buffer.extend(iter::repeat_n(value, count as usize / 2))),
        )
        .is_none()
        {
            -1
        } else if buffer.is_empty() {
            0
        } else {
            let half = buffer.len() / 2;

            buffer.select_nth_unstable(half);

            let double_min = min * 2;

            buffer[..half].iter().map(|&num| i64::from(num.min(double_min))).sum()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        Self::min_cost(basket1, basket2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
