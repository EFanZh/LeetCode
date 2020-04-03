pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        const DIGITS: [(&[u8], i32); 13] = [
            (b"M", 1000),
            (b"CM", 900),
            (b"D", 500),
            (b"CD", 400),
            (b"C", 100),
            (b"XC", 90),
            (b"L", 50),
            (b"XL", 40),
            (b"X", 10),
            (b"IX", 9),
            (b"V", 5),
            (b"IV", 4),
            (b"I", 1),
        ];

        let mut result = 0;
        let mut s = s.as_bytes();
        let (mut digit, mut digits) = DIGITS.split_first().unwrap();

        loop {
            if s.starts_with(digit.0) {
                result += digit.1;
                s = &s[digit.0.len()..];

                if s.is_empty() {
                    break;
                }
            } else {
                let (new_digit, new_digits) = digits.split_first().unwrap();

                digit = new_digit;
                digits = new_digits;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn roman_to_int(s: String) -> i32 {
        Self::roman_to_int(s)
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
