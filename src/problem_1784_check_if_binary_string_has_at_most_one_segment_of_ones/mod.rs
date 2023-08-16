pub mod iterative;

pub trait Solution {
    fn check_ones_segment(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1001", false), ("110", true), ("111", true)];

        for (s, expected) in test_cases {
            assert_eq!(S::check_ones_segment(s.to_string()), expected);
        }
    }
}
