pub mod iterative;

pub trait Solution {
    fn thousand_separator(n: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (987, "987"),
            (1234, "1.234"),
            (0, "0"),
            (1, "1"),
            (12, "12"),
            (123, "123"),
            (12345, "12.345"),
            (123_456, "123.456"),
            (1_234_567, "1.234.567"),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::thousand_separator(n), expected);
        }
    }
}
