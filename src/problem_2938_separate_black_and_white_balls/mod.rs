pub mod iterative;

pub trait Solution {
    fn minimum_steps(s: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("101", 1), ("100", 2), ("0111", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::minimum_steps(s.to_string()), expected);
        }
    }
}
