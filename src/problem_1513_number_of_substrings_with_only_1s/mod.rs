pub mod iterative;

pub trait Solution {
    fn num_sub(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("0110111", 9), ("101", 2), ("111111", 21)];

        for (s, expected) in test_cases {
            assert_eq!(S::num_sub(s.to_string()), expected);
        }
    }
}
