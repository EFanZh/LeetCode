pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_digit_occurrences(nums: Vec<i32>, digit: i32) -> i32 {
        let mut result = 0;

        for num in nums {
            let mut num = num.cast_unsigned();

            while num != 0 {
                let d = num % 10;

                num /= 10;
                result += i32::from(d == digit.cast_unsigned());
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_digit_occurrences(nums: Vec<i32>, digit: i32) -> i32 {
        Self::count_digit_occurrences(nums, digit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
