pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut bits = 0_u64;

        for &num in &nums {
            bits |= 1 << num;
        }

        let mut xor = 0;

        while bits != 0 {
            let num = bits.trailing_zeros();

            xor ^= num;
            bits ^= 1 << num;
        }

        nums.iter().fold(xor, |xor, &num| xor ^ num).cast_signed()
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
