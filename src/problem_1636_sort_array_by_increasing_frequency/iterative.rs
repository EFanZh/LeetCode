pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut frequencies = [0_u8; 201];

        for &num in &nums {
            frequencies[(num as usize).wrapping_add(100)] += 1;
        }

        nums.sort_unstable_by_key(|&num| (frequencies[(num as usize).wrapping_add(100)], Reverse(num)));

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        Self::frequency_sort(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
