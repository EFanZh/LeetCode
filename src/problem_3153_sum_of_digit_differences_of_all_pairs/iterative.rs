pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut cache = [(0_u32, [0_u32; 10]); 10];

        let cache = nums
            .first()
            .copied()
            .and_then(|first| cache.get_mut(..first.checked_ilog10()? as usize + 1))
            .unwrap_or(&mut []);

        let mut result = 0;

        for mut num in nums {
            for (total, details) in &mut *cache {
                let digit = num % 10;

                num /= 10;

                let digit = digit as usize;
                let digit_count = &mut details[digit];

                result += u64::from(*total - *digit_count);
                *total += 1;
                *digit_count += 1;
            }
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        Self::sum_digit_differences(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
