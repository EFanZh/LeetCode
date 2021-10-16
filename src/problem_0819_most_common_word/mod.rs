pub mod hash_map;
pub mod hash_map_2;

pub trait Solution {
    fn most_common_word(paragraph: String, banned: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    "Bob hit a ball, the hit BALL flew far after it was hit.",
                    &["hit"] as &[_],
                ),
                "ball",
            ),
            (("a.", &[]), "a"),
        ];

        for ((paragraph, banned), expected) in test_cases {
            assert_eq!(
                S::most_common_word(
                    paragraph.to_string(),
                    banned.iter().copied().map(str::to_string).collect()
                ),
                expected
            );
        }
    }
}
