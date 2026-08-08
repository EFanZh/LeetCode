pub mod greedy;

pub trait Solution {
    fn min_length_after_removals(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aabbab", 0), ("aaaa", 4), ("aaabb", 1)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_length_after_removals(s.to_string()), expected);
        }
    }
}
