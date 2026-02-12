pub mod iterative;

pub trait Solution {
    fn is_balanced(num: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1234", false), ("24123", true)];

        for (num, expected) in test_cases {
            assert_eq!(S::is_balanced(num.to_string()), expected);
        }
    }
}
