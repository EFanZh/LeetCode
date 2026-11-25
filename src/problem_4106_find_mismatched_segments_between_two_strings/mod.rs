pub mod iterative;

pub trait Solution {
    fn find_mismatched_segments(s1: String, s2: String) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("earth", "empty"), &[["ar", "mp"], ["h", "y"]] as &[_]),
            (("action", "nation"), &[["ac", "na"]]),
            (("hot", "hot"), &[]),
        ];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(S::find_mismatched_segments(s1.to_string(), s2.to_string()), expected);
        }
    }
}
