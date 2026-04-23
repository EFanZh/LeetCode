pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 {
        let mut x = x.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut buckets = HashMap::new();

        x.iter().zip(y).for_each(|(&key, value)| {
            let value = value.cast_unsigned();

            match buckets.entry(key) {
                Entry::Occupied(occupied_entry) => {
                    let max = occupied_entry.into_mut();

                    *max = u32::max(*max, value);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(value);
                }
            }
        });

        if buckets.len() < 3 {
            -1
        } else {
            x.clear();
            x.extend(buckets.values());
            x.select_nth_unstable_by(2, |lhs, rhs| u32::cmp(rhs, lhs));

            x[..3].iter().sum::<u32>().cast_signed()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 {
        Self::max_sum_distinct_triplet(x, y)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
