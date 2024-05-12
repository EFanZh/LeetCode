pub mod iterative;

pub trait Solution {
    fn count_palindromic_subsequence(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aabca", 3), ("adc", 0), ("bbcbaba", 4)];

        for (s, expected) in test_cases {
            assert_eq!(S::count_palindromic_subsequence(s.to_string()), expected);
        }
    }
}
