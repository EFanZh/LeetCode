pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Counter {
    counts: HashMap<u32, u32>,
    sum: u64,
}

impl Counter {
    fn add(&mut self, value: u32) {
        match self.counts.entry(value) {
            Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }

        self.sum += u64::from(value);
    }

    fn remove(&mut self, value: u32) {
        match self.counts.entry(value) {
            Entry::Occupied(occupied_entry) => {
                if *occupied_entry.get() == 1 {
                    occupied_entry.remove();
                } else {
                    *occupied_entry.into_mut() -= 1;
                }
            }
            Entry::Vacant(_) => unreachable!(),
        }

        self.sum -= u64::from(value);
    }
}

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();
        let k = k as usize;

        let mut counter = Counter {
            counts: HashMap::with_capacity(k),
            sum: 0,
        };

        let (left, right) = nums.split_at(k - 1);

        for &value in left {
            counter.add(value);
        }

        let mut result = 0;

        nums.iter().zip(right).for_each(|(&old, &new)| {
            counter.add(new);

            if counter.counts.len() == k {
                result = result.max(counter.sum);
            }

            counter.remove(old);
        });

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        Self::maximum_subarray_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;
    use std::collections::HashMap;

    #[test]
    #[should_panic = "internal error: entered unreachable code"]
    fn test_counter_remove() {
        let mut counter = Counter {
            counts: HashMap::new(),
            sum: 0,
        };

        counter.remove(7);
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
