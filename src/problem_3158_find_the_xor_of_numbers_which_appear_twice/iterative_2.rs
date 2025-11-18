pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut seen = 0_u64;
        let mut result = 0;

        for &num in &nums {
            let probe = 1 << num;

            if seen & probe == 0 {
                seen |= probe;
            } else {
                result ^= num;
            }
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        Self::duplicate_numbers_xor(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
