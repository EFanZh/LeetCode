pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let n = nums.len() as u32;
        let max = nums.iter().copied().fold(0, u32::max);
        let sum = nums.iter().sum::<u32>();

        (max * n - sum).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_moves(nums: Vec<i32>) -> i32 {
        Self::min_moves(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
