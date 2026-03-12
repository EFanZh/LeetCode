pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let mut prev = nums.last().copied().unwrap_or(0);
        let mut result = 0;

        for &num in &nums {
            result = u32::max(result, num.abs_diff(prev));
            prev = num;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        Self::max_adjacent_distance(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
