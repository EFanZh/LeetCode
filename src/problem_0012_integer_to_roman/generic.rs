pub struct Solution {}

use std::iter;

impl Solution {
    fn int_to_roman_helper(num: i32, base: i32, digits: (u8, u8, u8), result: &mut Vec<u8>) -> i32 {
        let digit = (num / base) as usize;

        match digit {
            0..=3 => result.extend(iter::repeat(digits.0).take(digit)),
            4 => result.extend(&[digits.0, digits.1]),
            5..=8 => {
                result.push(digits.1);
                result.extend(iter::repeat(digits.0).take(digit - 5));
            }
            9 => result.extend(&[digits.0, digits.2]),
            _ => unreachable!(),
        }

        num % base
    }

    pub fn int_to_roman(mut num: i32) -> String {
        let mut result = Vec::new();

        for _ in 0..num / 1000 {
            result.push(b'M');
        }

        num = Self::int_to_roman_helper(num % 1000, 100, (b'C', b'D', b'M'), &mut result);
        num = Self::int_to_roman_helper(num, 10, (b'X', b'L', b'C'), &mut result);
        Self::int_to_roman_helper(num, 1, (b'I', b'V', b'X'), &mut result);

        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn int_to_roman(num: i32) -> String {
        Self::int_to_roman(num)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
