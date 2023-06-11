pub mod iterative;

pub trait Solution {
    fn longest_awesome(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("3242415", 5), ("12345678", 1), ("213123", 6)];

        for (s, expected) in test_cases {
            assert_eq!(S::longest_awesome(s.to_string()), expected);
        }
    }
}
