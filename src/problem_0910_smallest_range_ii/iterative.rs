pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let first = *nums.first().unwrap();
        let last = *nums.last().unwrap();
        let mut result = last - first;

        for (&left, &right) in nums.iter().zip(&nums[1..]) {
            result = result.min((last - k).max(left + k) - (first + k).min(right - k));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        Self::smallest_range_ii(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
