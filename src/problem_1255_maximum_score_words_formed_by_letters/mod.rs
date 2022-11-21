pub mod brute_force;

pub trait Solution {
    fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["dog", "cat", "dad", "good"] as &[_],
                    "aacdddgoo",
                    [
                        1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                23,
            ),
            (
                (
                    &["xxxz", "ax", "bx", "cx"],
                    "zabcxxx",
                    [
                        4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
                    ],
                ),
                27,
            ),
            (
                (
                    &["leetcode"],
                    "letcod",
                    [
                        0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                    ],
                ),
                0,
            ),
        ];

        for ((words, letters, score), expected) in test_cases {
            assert_eq!(
                S::max_score_words(
                    words.iter().copied().map(str::to_string).collect(),
                    letters.chars().collect(),
                    score.to_vec(),
                ),
                expected
            );
        }
    }
}
