pub mod iterative;

pub trait Solution {
    fn num_ways(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("10101", 4), ("1001", 0), ("0000", 3), ("100100010100110", 12)];

        for (s, expected) in test_cases {
            assert_eq!(S::num_ways(s.to_string()), expected);
        }
    }
}
