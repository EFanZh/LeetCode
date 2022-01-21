pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

impl Solution {
    fn partition(nums: &mut [i32], pivot: i32) -> (usize, usize) {
        // | less | garbage | unknown | greater |
        //         ^         ^         ^
        //         i         j         k

        let mut i = 0;
        let mut j = 0;
        let mut k = nums.len();

        while j < k {
            let num = nums[j];

            match num.cmp(&pivot) {
                Ordering::Less => {
                    nums[i] = num;
                    i += 1;
                    j += 1;
                }
                Ordering::Equal => j += 1,
                Ordering::Greater => {
                    k -= 1;
                    nums.swap(j, k);
                }
            }
        }

        for target in &mut nums[i..j] {
            *target = pivot;
        }

        (i, k)
    }

    fn quick_sort_by(mut nums: &mut [i32], rng: &mut impl FnMut(&[i32]) -> usize) {
        while nums.len() > 1 {
            let pivot = nums[rng(nums) % nums.len()];
            let (split_1, split_2) = Self::partition(nums, pivot);

            let (left, rest) = nums.split_at_mut(split_1);
            let right = &mut rest[split_2 - split_1..];

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
