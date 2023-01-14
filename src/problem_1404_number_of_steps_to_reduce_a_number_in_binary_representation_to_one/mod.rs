pub mod iterative;

pub trait Solution {
    fn num_steps(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1101", 6), ("10", 1), ("1", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::num_steps(s.to_string()), expected);
        }
    }
}
