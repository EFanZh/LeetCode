pub mod stacks;

pub trait Solution {
    fn calculate_score(s: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aczzx", 5), ("abcdef", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::calculate_score(s.to_string()), expected);
        }
    }
}
