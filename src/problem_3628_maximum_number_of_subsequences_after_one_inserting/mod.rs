pub mod greedy;

pub trait Solution {
    fn num_of_subsequences(s: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("LMCT", 2), ("LCCT", 4), ("L", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::num_of_subsequences(s.to_string()), expected);
        }
    }
}
