pub mod brute_force;

pub trait Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["flower", "flow", "flight"] as &[_], "fl"),
            (&["dog", "racecar", "car"], ""),
            (&[], ""),
        ];

        for (strs, expected) in test_cases {
            assert_eq!(
                S::longest_common_prefix(strs.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
