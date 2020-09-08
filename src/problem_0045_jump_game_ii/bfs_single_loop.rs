pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
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
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
