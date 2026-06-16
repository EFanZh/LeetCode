pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut nums = nums.into_iter().map(i32::unsigned_abs).collect::<Vec<_>>();
        let half = nums.len() / 2;

        nums.select_nth_unstable(half);

        let (left, right) = nums.split_at(half);
        let right_sum = right.iter().map(|&x| u64::from(x).pow(2)).sum::<u64>();

        left.iter()
            .fold(right_sum, |result, &x| result - u64::from(x).pow(2))
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        Self::max_alternating_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
