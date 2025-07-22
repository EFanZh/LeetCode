pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_length_after_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len() as u32;
        let mut double_max_count = 0;
        let mut current_value = 0;
        let mut current_double_count = 0;

        for num in nums {
            if num == current_value {
                current_double_count += 2;
            } else {
                double_max_count = u32::max(double_max_count, current_double_count);
                current_value = num;
                current_double_count = 2;
            }
        }

        u32::max(double_max_count, current_double_count)
            .checked_sub(n)
            .unwrap_or(n & 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_length_after_removals(nums: Vec<i32>) -> i32 {
        Self::min_length_after_removals(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
