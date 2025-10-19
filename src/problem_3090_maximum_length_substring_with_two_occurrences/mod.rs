pub mod sliding_window;

pub trait Solution {
    fn maximum_length_substring(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("bcbbbcba", 4), ("aaaa", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::maximum_length_substring(s.to_string()), expected);
        }
    }
}
