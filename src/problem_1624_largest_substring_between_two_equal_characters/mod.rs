pub mod iterative;

pub trait Solution {
    fn max_length_between_equal_characters(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aa", 0), ("abca", 2), ("cbzxy", -1)];

        for (s, expected) in test_cases {
            assert_eq!(S::max_length_between_equal_characters(s.to_string()), expected);
        }
    }
}
