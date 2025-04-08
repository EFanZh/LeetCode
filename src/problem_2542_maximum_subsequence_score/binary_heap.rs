pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::{iter, mem};

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as u32 as usize;

        let mut nums = iter::zip(nums1, nums2)
            .map(|(num_1, num_2)| (num_1 as u32, num_2 as u32))
            .collect::<Box<_>>();

        nums.sort_unstable_by_key(|&(_, num_2)| num_2);

        let mut sum = 0;

        let mut heap = nums[nums.len() - k..]
            .iter()
            .map(|&(num_1, _)| {
                sum += u64::from(num_1);

                Reverse(num_1)
            })
            .collect::<BinaryHeap<_>>();

        let mut result = sum * u64::from(nums[nums.len() - k].1);

        for &(num_1, num_2) in nums[..nums.len() - k].iter().rev() {
            let old_top = mem::replace(&mut heap.peek_mut().unwrap().0, num_1);

            sum += u64::from(num_1);
            sum -= u64::from(old_top);
            result = result.max(sum * u64::from(num_2));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        Self::max_score(nums1, nums2, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
