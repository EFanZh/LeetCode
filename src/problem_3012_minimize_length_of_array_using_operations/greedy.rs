pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    pub fn minimum_array_length(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let min = NonZero::new(nums.iter().copied().fold(u32::MAX, u32::min)).unwrap();
        let mut min_count = 0;

        for &num in &nums {
            if num % min != 0 {
                return 1;
            }

            min_count += u32::from(num == min.get());
        }

        min_count.div_ceil(2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_array_length(nums: Vec<i32>) -> i32 {
        Self::minimum_array_length(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
