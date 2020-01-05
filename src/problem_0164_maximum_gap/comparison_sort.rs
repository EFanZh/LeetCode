pub struct Solution {}

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut iter = nums.into_iter();
        let mut result = 0;

        if let Some(mut previous) = iter.next() {
            for current in iter {
                result = result.max(current - previous);

                previous = current
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn maximum_gap(nums: Vec<i32>) -> i32 {
        Self::maximum_gap(nums)
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
