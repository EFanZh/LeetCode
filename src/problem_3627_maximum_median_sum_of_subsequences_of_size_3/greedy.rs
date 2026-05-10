pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_median_sum(nums: Vec<i32>) -> i64 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let split = nums.len() / 3;

        nums.select_nth_unstable(split);

        let right = &mut nums[split..];

        right.sort_unstable();

        right
            .iter()
            .step_by(2)
            .copied()
            .map(u64::from)
            .sum::<u64>()
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_median_sum(nums: Vec<i32>) -> i64 {
        Self::maximum_median_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
