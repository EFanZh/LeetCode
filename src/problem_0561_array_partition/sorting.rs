pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        nums.iter().step_by(2).sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn array_pair_sum(nums: Vec<i32>) -> i32 {
        Self::array_pair_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
