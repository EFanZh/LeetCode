pub mod sliding_window;

pub trait Solution {
    fn max_consecutive_answers(answer_key: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("TTFF", 2), 4), (("TFFT", 1), 3), (("TTFTTFTT", 1), 5)];

        for ((answer_key, k), expected) in test_cases {
            assert_eq!(S::max_consecutive_answers(answer_key.to_string(), k), expected);
        }
    }
}
