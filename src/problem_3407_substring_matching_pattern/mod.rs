pub mod greedy;

pub trait Solution {
    fn has_match(s: String, p: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("leetcode", "ee*e"), true),
            (("car", "c*v"), false),
            (("luck", "u*"), true),
        ];

        for ((s, p), expected) in test_cases {
            assert_eq!(S::has_match(s.to_string(), p.to_string()), expected);
        }
    }
}
