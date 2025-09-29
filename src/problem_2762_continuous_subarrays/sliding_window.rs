pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut top_length = 0_u64;
        let mut middle_length = 0_u64;
        let mut bottom_length = 0_u64;
        let mut prev = 0;
        let mut result = 0;

        for num in nums {
            let diff = num - mem::replace(&mut prev, num);

            (top_length, middle_length, bottom_length) = match diff {
                -2 => (1, 1, top_length + 1),
                -1 => (1, top_length + 1, middle_length + 1),
                0 => (top_length + 1, middle_length + 1, bottom_length + 1),
                1 => (middle_length + 1, bottom_length + 1, 1),
                2 => (bottom_length + 1, 1, 1),
                _ => (1, 1, 1),
            };

            result += top_length.max(middle_length).max(bottom_length);
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        Self::continuous_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
