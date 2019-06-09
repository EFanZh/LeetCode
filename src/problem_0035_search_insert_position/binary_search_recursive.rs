pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        fn helper(nums: &[i32], target: i32, offset: usize) -> usize {
            if nums.is_empty() {
                offset
            } else {
                let middle = nums.len() / 2;

                if nums[middle] < target {
                    helper(&nums[middle + 1..], target, offset + middle + 1)
                } else {
                    helper(&nums[..middle], target, offset)
                }
            }
        }

        helper(&nums, target, 0) as _
    }
}

impl super::Solution for Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Solution::search_insert(nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
