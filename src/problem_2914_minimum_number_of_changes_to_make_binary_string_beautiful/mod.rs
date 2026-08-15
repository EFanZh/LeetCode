pub mod greedy;

pub trait Solution {
    fn min_changes(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1001", 2), ("10", 1), ("0000", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_changes(s.to_string()), expected);
        }
    }
}
