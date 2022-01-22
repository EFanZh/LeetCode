pub mod iterative;

pub trait Solution {
    fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("1357", 100), 20),
            (("149", 1_000_000_000), 29523),
            (("7", 8), 1),
            (("17", 231), 10),
        ];

        for ((digits, n), expected) in test_cases {
            assert_eq!(
                S::at_most_n_given_digit_set(digits.chars().map(|c| c.to_string()).collect(), n),
                expected
            );
        }
    }
}
