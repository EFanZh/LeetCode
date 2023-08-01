pub mod iterative;

pub trait Solution {
    fn reformat_number(number: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("1-23-45 6", "123-456"),
            ("123 4-567", "123-45-67"),
            ("123 4-5678", "123-456-78"),
        ];

        for (number, expected) in test_cases {
            assert_eq!(S::reformat_number(number.to_string()), expected);
        }
    }
}
