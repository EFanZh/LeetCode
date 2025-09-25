pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::mem;

impl Solution {
    fn div_mod(lhs: u32, rhs: u32) -> (u32, u32) {
        (lhs / rhs, lhs % rhs)
    }

    fn to_digits(value: u32) -> (u64, u8) {
        let mut result = 0;
        let mut length = 0;
        let mut value = value;
        let mut shift = 0_u8;

        while value != 0 {
            let digit;

            (value, digit) = Self::div_mod(value, 10);
            result |= u64::from(digit) << shift;
            length += 1;
            shift += 4;
        }

        (result, length)
    }

    fn to_digits_vec(values: Vec<i32>) -> (Vec<u64>, u8) {
        let mut max_length = 0;

        let digits_vec = values
            .into_iter()
            .map(|value| {
                let (digits, length) = Self::to_digits(value.cast_unsigned());

                max_length = max_length.max(length);

                digits | (u64::from(length) << 60)
            })
            .collect();

        (digits_vec, max_length)
    }

    fn iter_digits(digits: &[u64], prefix_length: u8) -> impl Iterator<Item = u64> {
        digits.iter().filter_map(move |&digits| {
            let length = (digits >> 60) as u8;

            if length < prefix_length {
                None
            } else {
                Some((digits & !(0b_1111 << 60)) >> ((length - prefix_length) * 4))
            }
        })
    }

    fn check(digits_1: &[u64], digits_2: &[u64], buffer: &mut HashSet<u64>, middle: u8) -> bool {
        buffer.extend(Self::iter_digits(digits_1, middle));

        let result = Self::iter_digits(digits_2, middle).any(|digits| buffer.contains(&digits));

        buffer.clear();

        result
    }

    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr1 = arr1;
        let mut arr2 = arr2;

        if arr2.len() < arr1.len() {
            mem::swap(&mut arr1, &mut arr2);
        }

        let (digits_1, max_length_1) = Self::to_digits_vec(arr1);
        let (digits_2, max_length_2) = Self::to_digits_vec(arr2);
        let mut left = 1;
        let mut right = max_length_1.min(max_length_2) + 1;
        let mut buffer = HashSet::new();

        while left != right {
            let middle = left.midpoint(right);

            if Self::check(&digits_1, &digits_2, &mut buffer, middle) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        i32::from(left - 1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        Self::longest_common_prefix(arr1, arr2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
