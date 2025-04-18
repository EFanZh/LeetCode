pub mod prefix_sums;

pub trait Solution {
    fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["aba", "bcb", "ece", "aa", "e"] as &[_],
                    &[[0, 2], [1, 4], [1, 1]] as &[_],
                ),
                &[2, 3, 0] as &[_],
            ),
            ((&["a", "e", "i"], &[[0, 2], [0, 1], [2, 2]]), &[3, 2, 1]),
        ];

        for ((words, queries), expected) in test_cases {
            assert_eq!(
                S::vowel_strings(
                    words.iter().copied().map(str::to_string).collect(),
                    queries.iter().map(Vec::from).collect(),
                ),
                expected,
            );
        }
    }
}
