pub mod greedy;

pub trait Solution {
    fn min_length(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("ABFCACDB", 2), ("ACBBD", 5)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_length(s.to_string()), expected);
        }
    }
}
