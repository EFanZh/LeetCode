pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut size = nums.len();

        while size > 0 {
            let half = size / 2;

            if nums[left + half] < target {
                left += size - half;
            }

            size = half;
        }

        left as _
    }
}

impl super::Solution for Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_insert(nums, target)
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
