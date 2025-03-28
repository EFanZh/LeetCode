pub mod greedy;

pub trait Solution {
    fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["hello", "i", "am", "leetcode", "hello"] as &[_], "hello", 1), 1),
            ((&["a", "b", "leetcode"], "leetcode", 0), 1),
            ((&["i", "eat", "leetcode"], "ate", 0), -1),
        ];

        for ((words, target, start_index), expected) in test_cases {
            assert_eq!(
                S::closest_target(
                    words.iter().copied().map(str::to_string).collect(),
                    target.to_string(),
                    start_index,
                ),
                expected,
            );
        }
    }
}
