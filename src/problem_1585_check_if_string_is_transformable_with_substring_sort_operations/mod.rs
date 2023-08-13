pub mod iterative;

pub trait Solution {
    fn is_transformable(s: String, t: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("84532", "34852"), true),
            (("34521", "23415"), true),
            (("12345", "12435"), false),
            (("1", "2"), false),
        ];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::is_transformable(s.to_string(), t.to_string()), expected);
        }
    }
}
