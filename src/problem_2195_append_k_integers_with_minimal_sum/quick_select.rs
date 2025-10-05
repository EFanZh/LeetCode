pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{BuildHasherDefault, DefaultHasher};
use std::mem;

impl Solution {
    fn partition(values: &mut [u32], key: u32) -> usize {
        let mut split = 0;
        let mut iter = values.iter_mut();

        'outer: loop {
            let left = loop {
                if let Some(left) = iter.next() {
                    if *left < key {
                        split += 1;
                    } else {
                        break left;
                    }
                } else {
                    break 'outer;
                }
            };

            loop {
                if let Some(right) = iter.next_back() {
                    if *right < key {
                        mem::swap(left, right);

                        split += 1;

                        break;
                    }
                } else {
                    break 'outer;
                }
            }
        }

        split
    }

    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut k = u64::from(k as u32);

        let nums_set = nums
            .iter()
            .copied()
            .collect::<HashSet<_, BuildHasherDefault<DefaultHasher>>>();

        nums.splice(.., nums_set);

        let mut left = 0;
        let mut length = nums.len();

        while length != 0 {
            let window = &mut nums[left..left + length];
            let (&mut key, rest) = window.split_first_mut().unwrap();
            let split = Self::partition(rest, key);

            window.swap(0, split);

            let free = key - 1 - (left as u32 + split as u32);

            match free.cmp(&(k as u32)) {
                Ordering::Less => {
                    left += split + 1;
                    length -= split + 1;
                }
                Ordering::Equal => {
                    left += split;

                    break;
                }
                Ordering::Greater => length = split,
            }
        }

        k += left as u64;

        (k * (k + 1) / 2 - nums[..left].iter().fold(0, |sum, &x| sum + u64::from(x))) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        Self::minimal_k_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
