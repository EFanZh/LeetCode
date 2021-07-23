pub mod iterative;

pub trait Solution {
    fn count_binary_substrings(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("00110011", 6), ("10101", 4)];

        for (s, expected) in test_cases {
            assert_eq!(S::count_binary_substrings(s.to_string()), expected);
        }
    }
}
