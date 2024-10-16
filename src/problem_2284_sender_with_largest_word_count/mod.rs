pub mod hash_map;

pub trait Solution {
    fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[
                        "Hello userTwooo",
                        "Hi userThree",
                        "Wonderful day Alice",
                        "Nice day userThree",
                    ] as &[_],
                    &["Alice", "userTwo", "userThree", "Alice"] as &[_],
                ),
                "Alice",
            ),
            (
                (
                    &["How is leetcode for everyone", "Leetcode is useful for practice"],
                    &["Bob", "Charlie"],
                ),
                "Charlie",
            ),
        ];

        for ((messages, senders), expected) in test_cases {
            assert_eq!(
                S::largest_word_count(
                    messages.iter().copied().map(str::to_string).collect(),
                    senders.iter().copied().map(str::to_string).collect(),
                ),
                expected,
            );
        }
    }
}
