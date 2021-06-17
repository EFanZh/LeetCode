pub mod iterative;

pub trait Solution {
    fn fizz_buzz(n: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, &["1"] as &[_]),
            (
                15,
                &[
                    "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14",
                    "FizzBuzz",
                ],
            ),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::fizz_buzz(n), expected);
        }
    }
}
