pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str;

impl Solution {
    fn get_letter(digit: u8) -> &'static [u8] {
        match digit {
            b'2' => b"abc",
            b'3' => b"def",
            b'4' => b"ghi",
            b'5' => b"jkl",
            b'6' => b"mno",
            b'7' => b"pqrs",
            b'8' => b"tuv",
            _ => b"wxyz",
        }
    }

    fn push_to_result(result: &mut Vec<String>, item: &[u8]) {
        result.push(str::from_utf8(item).unwrap().to_string());
    }

    fn no_op(_: &mut Vec<String>, _: &[u8]) {}

    fn enumerate_combinations(
        digits: &[u8],
        prefix: &mut Vec<u8>,
        result: &mut Vec<String>,
        operation: impl FnOnce(&mut Vec<String>, &[u8]),
    ) {
        if let Some((&first, rest)) = digits.split_first() {
            for &letter in Self::get_letter(first) {
                prefix.push(letter);

                Self::enumerate_combinations(rest, prefix, result, Self::push_to_result);

                prefix.pop();
            }
        } else {
            operation(result, prefix);
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits = digits.into_bytes();
        let mut result = Vec::new();

        Self::enumerate_combinations(&digits, &mut Vec::new(), &mut result, Self::no_op);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn letter_combinations(digits: String) -> Vec<String> {
        Self::letter_combinations(digits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
