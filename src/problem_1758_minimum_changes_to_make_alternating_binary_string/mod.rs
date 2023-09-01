pub mod dynamic_programming;
pub mod iterative;

pub trait Solution {
    fn min_operations(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("0100", 1), ("10", 0), ("1111", 2), ("110010", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_operations(s.to_string()), expected);
        }
    }
}
