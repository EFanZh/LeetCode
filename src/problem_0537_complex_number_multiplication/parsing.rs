pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse_integer(input: &[u8]) -> (i16, &[u8]) {
        let (&first, mut input) = input.split_first().unwrap();

        if first == b'-' {
            let mut result = 0;

            while let Some((&c @ b'0'..=b'9', rest)) = input.split_first() {
                result = result * 10 - i16::from(c - b'0');
                input = rest;
            }

            (result, input)
        } else {
            let mut result = i16::from(first - b'0');

            while let Some((&c @ b'0'..=b'9', rest)) = input.split_first() {
                result = result * 10 + i16::from(c - b'0');
                input = rest;
            }

            (result, input)
        }
    }

    fn parse_complex(input: &[u8]) -> (i16, i16) {
        let (real, input) = Self::parse_integer(input);
        let (imaginary, _) = Self::parse_integer(&input[1..]);

        (real, imaginary)
    }

    pub fn complex_number_multiply(a: String, b: String) -> String {
        use std::fmt::Write;

        let (real_1, imaginary_1) = Self::parse_complex(a.as_bytes());
        let (real_2, imaginary_2) = Self::parse_complex(b.as_bytes());
        let result_real = real_1 * real_2 - imaginary_1 * imaginary_2;
        let result_imaginary = real_1 * imaginary_2 + imaginary_1 * real_2;
        let mut result = if a.len() < b.len() { b } else { a };

        result.clear();

        write!(&mut result, "{}+{}i", result_real, result_imaginary).unwrap();

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn complex_number_multiply(a: String, b: String) -> String {
        Self::complex_number_multiply(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
