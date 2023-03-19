pub mod iterative;

pub trait Solution {
    fn num_splits(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aacaba", 2), ("abcd", 1)];

        for (s, expected) in test_cases {
            assert_eq!(S::num_splits(s.to_string()), expected);
        }
    }
}
