pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        nums.iter()
            .map(|&num| {
                let mut num = num.cast_unsigned();
                let mut max_digit = 0;
                let mut digit_count = 0;

                while num != 0 {
                    let digit = num % 10;

                    num /= 10;

                    max_digit = max_digit.max(digit);
                    digit_count += 1;
                }

                ((u32::pow(10, digit_count) - 1) / 9 * max_digit).cast_signed()
            })
            .sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        Self::sum_of_encrypted_int(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
