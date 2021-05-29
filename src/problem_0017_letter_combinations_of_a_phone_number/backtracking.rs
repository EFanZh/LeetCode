pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

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

    fn enumerate_combinations(digits: &[u8], prefix: &mut Vec<u8>, result: &mut Vec<String>) {
        if let Some((first, rest)) = digits.split_first() {
            for letter in Self::get_letter(*first) {
                prefix.push(*letter);

                Self::enumerate_combinations(rest, prefix, result);

                prefix.pop();
            }
        } else {
            result.push(String::from_utf8(prefix.clone()).unwrap());
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits = digits.into_bytes();
        let mut result = Vec::new();

        if !digits.is_empty() {
            let mut prefix = Vec::new();

            Self::enumerate_combinations(&digits, &mut prefix, &mut result);
        }

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
