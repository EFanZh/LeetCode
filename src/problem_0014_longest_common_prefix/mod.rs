pub mod brute_force;

pub trait Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [
            (&["flower", "flow", "flight"] as &[_], "fl"),
            (&["dog", "racecar", "car"], ""),
            (&[], ""),
        ];

        for (strs, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::longest_common_prefix(strs.iter().copied().map(ToString::to_string).collect()),
                expected
            );
        }
    }
}
