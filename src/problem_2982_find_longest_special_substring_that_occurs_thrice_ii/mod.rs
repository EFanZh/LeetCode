pub mod greedy;

pub trait Solution {
    fn maximum_length(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aaaa", 2), ("abcdef", -1), ("abcaba", 1)];

        for (s, expected) in test_cases {
            assert_eq!(S::maximum_length(s.to_string()), expected);
        }
    }
}
