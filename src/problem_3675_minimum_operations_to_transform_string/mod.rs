pub mod greedy;

pub trait Solution {
    fn min_operations(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("yz", 2), ("a", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_operations(s.to_string()), expected);
        }
    }
}
