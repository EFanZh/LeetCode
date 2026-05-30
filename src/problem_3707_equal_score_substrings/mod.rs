pub mod iterative;

pub trait Solution {
    fn score_balance(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("adcb", true), ("bace", false), ("edb", false)];

        for (s, expected) in test_cases {
            assert_eq!(S::score_balance(s.to_string()), expected);
        }
    }
}
