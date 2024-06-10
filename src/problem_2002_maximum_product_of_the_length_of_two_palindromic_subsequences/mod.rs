pub mod dynamic_programming;

pub trait Solution {
    fn max_product(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("leetcodecom", 9), ("bb", 1), ("accbcaxxcxx", 25)];

        for (s, expected) in test_cases {
            assert_eq!(S::max_product(s.to_string()), expected);
        }
    }
}
