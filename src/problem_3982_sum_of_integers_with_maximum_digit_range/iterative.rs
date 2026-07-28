pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn digit_range(x: i32) -> u32 {
        let mut x = x.cast_unsigned();
        let mut min = x % 10;
        let mut max = min;

        loop {
            x /= 10;

            if x == 0 {
                return max - min;
            }

            let digit = x % 10;

            if digit < min {
                min = digit;
            } else if digit > max {
                max = digit;
            }
        }
    }

    pub fn max_digit_range(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max_digit_range = 0;

        for num in nums {
            let digit_range = Self::digit_range(num);

            match digit_range.cmp(&max_digit_range) {
                Ordering::Less => {}
                Ordering::Equal => result += num,
                Ordering::Greater => {
                    max_digit_range = digit_range;
                    result = num;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_digit_range(nums: Vec<i32>) -> i32 {
        Self::max_digit_range(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
