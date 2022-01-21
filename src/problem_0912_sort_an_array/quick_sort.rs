pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

impl Solution {
    fn partition_by(nums: &mut [i32], mut f: impl FnMut(i32) -> bool) -> usize {
        let mut iter = nums.iter_mut();
        let mut i = 0;

        'outer: loop {
            let left = loop {
                if let Some(left) = iter.next() {
                    if f(*left) {
                        i += 1;
                    } else {
                        break left;
                    }
                } else {
                    break 'outer;
                }
            };

            loop {
                if let Some(right) = iter.next_back() {
                    if f(*right) {
                        i += 1;
                        mem::swap(left, right);

                        break;
                    }
                } else {
                    break 'outer;
                }
            }
        }

        i
    }

    fn quick_sort_by(mut nums: &mut [i32], rng: &mut impl FnMut(&[i32]) -> usize) {
        while nums.len() > 1 {
            nums.swap(0, rng(nums) % nums.len());

            let (&mut pivot, rest) = nums.split_first_mut().unwrap();
            let split = Self::partition_by(rest, |num| num < pivot);

            nums.swap(0, split);

            let (left, rest) = nums.split_at_mut(split);
            let right = &mut rest[1..];

            if right.len() < left.len() {
                Self::quick_sort_by(right, rng);
                nums = left;
            } else {
                Self::quick_sort_by(left, rng);
                nums = right;
            }
        }
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut rng = DefaultHasher::new();
        let mut nums = nums;

        Self::quick_sort_by(&mut nums, &mut |nums| {
            nums.len().hash(&mut rng);

            rng.finish() as _
        });

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
