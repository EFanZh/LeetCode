pub mod greedy;

pub trait Solution {
    fn max_operations(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1001101", 4), ("00111", 0), ("110", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::max_operations(s.to_string()), expected);
        }
    }
}
