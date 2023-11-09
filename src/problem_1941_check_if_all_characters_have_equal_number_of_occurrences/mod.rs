pub mod iterative;

pub trait Solution {
    fn are_occurrences_equal(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abacbc", true), ("aaabb", false)];

        for (s, expected) in test_cases {
            assert_eq!(S::are_occurrences_equal(s.to_string()), expected);
        }
    }
}
