pub mod iterative;

pub trait Solution {
    fn check_zero_ones(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1101", true), ("111000", false), ("110100010", false)];

        for (s, expected) in test_cases {
            assert_eq!(S::check_zero_ones(s.to_string()), expected);
        }
    }
}
