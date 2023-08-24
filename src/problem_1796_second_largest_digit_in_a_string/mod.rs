pub mod iterative;

pub trait Solution {
    fn second_highest(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("dfa12321afd", 2),
            ("abc1111", -1),
            ("xyz", -1),
            ("sjhtz8344", 4),
            ("1a0", 0),
            ("ck077", 0),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::second_highest(s.to_string()), expected);
        }
    }
}
