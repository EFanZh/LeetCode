pub struct Solution {}

impl Solution {
    fn jump(nums: Vec<i32>) -> i32 {
        let mut step = 0;
        let mut left = 0;
        let mut right = 1;

        while right < nums.len() {
            let farthest = nums
                .iter()
                .enumerate()
                .take(right)
                .skip(left)
                .map(|(i, n)| i + *n as usize)
                .max()
                .unwrap();

            step += 1;
            left = right;
            right = farthest + 1;
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
