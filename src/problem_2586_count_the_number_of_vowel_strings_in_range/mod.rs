pub mod iterative;

pub trait Solution {
    fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["are", "amy", "u"] as &[_], 0, 2), 2),
            ((&["hey", "aeo", "mu", "ooo", "artro"], 1, 4), 3),
        ];

        for ((words, left, right), expected) in test_cases {
            assert_eq!(
                S::vowel_strings(words.iter().copied().map(str::to_string).collect(), left, right),
                expected,
            );
        }
    }
}
