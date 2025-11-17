pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let digits = nums.first().copied().and_then(u32::checked_ilog10).map_or(0, |x| x + 1);
        let mut result = 0;

        for _ in 0..digits {
            let mut cache = [0_u32; 10];

            (0..).zip(&mut nums).for_each(|(total, num)| {
                let digit = *num % 10;

                *num /= 10;

                let digit_count = &mut cache[digit as usize];

                result += u64::from(total - *digit_count);
                *digit_count += 1;
            });
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
