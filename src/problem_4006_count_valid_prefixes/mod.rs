pub mod iterative;

pub trait Solution {
    fn count_valid_prefixes(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("00101", 3), ("101", 3)];

        for (s, expected) in test_cases {
            assert_eq!(S::count_valid_prefixes(s.to_string()), expected);
        }
    }
}
