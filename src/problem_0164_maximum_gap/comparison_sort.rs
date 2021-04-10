pub struct Solution;

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        nums.iter()
            .zip(&nums[1..])
            .map(|(previous, current)| current - previous)
            .max()
            .unwrap_or(0)
    }
}

impl super::Solution for Solution {
    fn maximum_gap(nums: Vec<i32>) -> i32 {
        Self::maximum_gap(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
