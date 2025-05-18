pub mod iterative;

pub trait Solution {
    fn find_the_longest_balanced_substring(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("01000111", 6), ("00111", 4), ("111", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::find_the_longest_balanced_substring(s.to_string()), expected);
        }
    }
}
