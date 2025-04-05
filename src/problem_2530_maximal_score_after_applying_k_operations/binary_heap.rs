pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.into_iter().map(|num| num as u32).collect::<BinaryHeap<_>>();
        let mut result = 0;

        for _ in 0..k as u32 {
            let max = &mut *nums.peek_mut().unwrap();

            result += u64::from(*max);
            *max = (*max + 2) / 3;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        Self::max_kelements(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
