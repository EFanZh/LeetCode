pub mod iterative;

pub trait Solution {
    fn max_score(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("011101", 5), ("00111", 5), ("1111", 3)];

        for (s, expected) in test_cases {
            assert_eq!(S::max_score(s.to_string()), expected);
        }
    }
}
