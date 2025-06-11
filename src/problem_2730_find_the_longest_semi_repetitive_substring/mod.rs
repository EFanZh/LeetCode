pub mod dynamic_programming;
pub mod sliding_window;

pub trait Solution {
    fn longest_semi_repetitive_substring(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("52233", 4), ("5494", 4), ("1111111", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::longest_semi_repetitive_substring(s.to_string()), expected);
        }
    }
}
