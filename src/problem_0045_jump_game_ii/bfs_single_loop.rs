pub struct Solution {}

impl Solution {
    fn jump(nums: Vec<i32>) -> i32 {
        let mut step = 0;
        let mut right = 1;
        let mut farthest = 0;
        let mut i = 0;

        while right < nums.len() {
            farthest = farthest.max(i + nums[i] as usize);

            i += 1;

            if i == right {
                step += 1;

                right = farthest + 1;
            }
        }

        step
    }
}

impl super::Solution for Solution {
    fn jump(nums: Vec<i32>) -> i32 {
        Self::jump(nums)
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
