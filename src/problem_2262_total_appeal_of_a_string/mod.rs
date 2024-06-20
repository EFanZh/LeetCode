pub mod dynamic_programming;

pub trait Solution {
    fn appeal_sum(s: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abbca", 28), ("code", 20)];

        for (s, expected) in test_cases {
            assert_eq!(S::appeal_sum(s.to_string()), expected);
        }
    }
}
