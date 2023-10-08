pub mod greedy;

pub trait Solution {
    fn min_swaps(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("111000", 1), ("010", 0), ("1110", -1), ("1", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_swaps(s.to_string()), expected);
        }
    }
}
