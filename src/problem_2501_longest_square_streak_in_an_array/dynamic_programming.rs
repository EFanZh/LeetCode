pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();

        nums.sort_unstable_by(|lhs, rhs| rhs.cmp(lhs));

        let mut lengths = HashMap::new();
        let mut result = 0_u32;

        for num in nums {
            let length = num
                .checked_mul(num)
                .and_then(|squared| lengths.get(&squared))
                .copied()
                .unwrap_or(0)
                + 1;

            lengths.insert(num, length);
            result = result.max(length);
        }

        if result < 2 { -1 } else { result as _ }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_square_streak(nums: Vec<i32>) -> i32 {
        Self::longest_square_streak(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
