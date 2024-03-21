pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut num = num as u32;
        let mut digits = [0; 10];
        let mut digits_len = 0;
        let mut even_digits = ([0; 10], 0);
        let mut odd_digits = ([0; 10], 0);

        while num != 0 {
            let digit = (num % 10) as u8;

            num /= 10;

            digits[digits_len] = digit;
            digits_len += 1;

            let digits = if digit % 2 == 0 {
                &mut even_digits
            } else {
                &mut odd_digits
            };

            digits.0[digits.1] = digit;
            digits.1 += 1;
        }

        let digits = &digits[..digits_len];
        let even_digits = &mut even_digits.0[..even_digits.1];
        let odd_digits = &mut odd_digits.0[..odd_digits.1];
        let reverse = |&x: &_| Reverse(x);

        even_digits.sort_unstable_by_key(reverse);
        odd_digits.sort_unstable_by_key(reverse);

        let mut even_iter = even_digits.iter().copied().map(u32::from);
        let mut odd_iter = odd_digits.iter().copied().map(u32::from);

        let mut result = 0;

        for &digit in digits.iter().rev() {
            let iter = if digit % 2 == 0 {
                even_iter.by_ref()
            } else {
                odd_iter.by_ref()
            };

            result = result * 10 + iter.next().unwrap();
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_integer(num: i32) -> i32 {
        Self::largest_integer(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
