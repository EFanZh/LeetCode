pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        nums.iter()
            .zip(&nums[1..])
            .map(|(previous, current)| current - previous)
            .max()
            .unwrap_or(0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
