pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut extra_xor_missing = 0;
        let mut i = 1;

        for &num in &nums {
            extra_xor_missing ^= num ^ i;
            i += 1;
        }

        let last_different_bit = extra_xor_missing & !(extra_xor_missing - 1);
        let mut zero_bit = 0;
        let mut one_bit = 0;

        i = 1;

        for &num in &nums {
            if num & last_different_bit == 0 {
                zero_bit ^= num;
            } else {
                one_bit ^= num;
            }

            if i & last_different_bit == 0 {
                zero_bit ^= i;
            } else {
                one_bit ^= i;
            }

            i += 1;
        }

        if nums.contains(&zero_bit) {
            vec![zero_bit, one_bit]
        } else {
            vec![one_bit, zero_bit]
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        Self::find_error_nums(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
