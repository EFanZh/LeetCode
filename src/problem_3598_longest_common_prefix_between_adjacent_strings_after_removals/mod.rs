pub mod iterative;

pub trait Solution {
    fn longest_common_prefix(words: Vec<String>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["jump", "run", "run", "jump", "run"] as &[_], &[3, 0, 0, 3, 3] as &[_]),
            (&["dog", "racer", "car"], &[0, 0, 0]),
            (&["cdbff"], &[0]),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::longest_common_prefix(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
