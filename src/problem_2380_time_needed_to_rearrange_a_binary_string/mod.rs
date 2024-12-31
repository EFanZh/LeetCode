pub mod iterative;

pub trait Solution {
    fn seconds_to_remove_occurrences(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("0110101", 4), ("11100", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::seconds_to_remove_occurrences(s.to_string()), expected);
        }
    }
}
