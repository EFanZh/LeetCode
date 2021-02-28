pub struct Solution;

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

impl Solution {
    fn select_pivot(length: usize, rng: &mut impl Hasher) -> usize {
        length.hash(rng);

        (rng.finish() % (length as u64)) as _
    }

    fn partition(nums: &mut [i32], rng: &mut impl Hasher) -> usize {
        let length = nums.len();
        let pivot = Self::select_pivot(nums.len(), rng);

        nums.swap(pivot, length - 1);

        let mut left = 0;
        let mut right = length - 1;
        let key = nums[right];

        'outer: while left != right {
            if nums[left] < key {
                left += 1;
            } else {
                loop {
                    if left + 1 == right {
                        break 'outer;
                    }

                    right -= 1;

                    if nums[right] < key {
                        nums.swap(left, right);
                        left += 1;

                        break;
                    }
                }
            }
        }

        nums.swap(left, length - 1);

        left
    }

    fn select_nth(mut nums: &mut [i32], mut n: usize) {
        let mut rng = DefaultHasher::new();

        while nums.len() > 1 {
            let pivot = Self::partition(nums, &mut rng);

            match pivot.cmp(&n) {
                Ordering::Less => {
                    nums = &mut nums[pivot + 1..];
                    n -= pivot + 1;
                }
                Ordering::Equal => break,
                Ordering::Greater => nums = &mut nums[..pivot],
            }
        }
    }

    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            0
        } else {
            let half = nums.len() / 2;

            Self::select_nth(&mut nums, half);

            let median = nums[half];

            nums.iter().map(|num| (num - median).abs()).sum()
        }
    }
}

impl super::Solution for Solution {
    fn min_moves2(nums: Vec<i32>) -> i32 {
        Self::min_moves2(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
