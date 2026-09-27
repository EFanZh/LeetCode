pub mod hash_map;

pub trait Solution {
    fn prefix_connected(words: Vec<String>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["apple", "apply", "banana", "bandit"] as &[_], 2), 2),
            ((&["car", "cat", "cartoon"], 3), 1),
            ((&["bat", "dog", "dog", "doggy", "bat"], 3), 2),
        ];

        for ((words, k), expected) in test_cases {
            assert_eq!(
                S::prefix_connected(words.iter().copied().map(str::to_string).collect(), k),
                expected,
            );
        }
    }
}
