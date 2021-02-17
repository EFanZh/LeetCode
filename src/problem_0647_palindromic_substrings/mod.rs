pub mod brute_force;

pub trait Solution {
    fn count_substrings(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("", 0), ("a", 1), ("abc", 3), ("aaa", 6), ("aba", 4)];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::count_substrings(s.to_string()), expected);
        }
    }
}
