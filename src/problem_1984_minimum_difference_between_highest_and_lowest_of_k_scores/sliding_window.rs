pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        nums.iter()
            .zip(&nums[k as u32 as usize - 1..])
            .fold(i32::MAX, |result, (left, right)| result.min(right - left))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        Self::minimum_difference(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
