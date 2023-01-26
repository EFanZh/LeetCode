pub mod compare_count;
pub mod sort_and_compare;

pub trait Solution {
    fn check_if_can_break(s1: String, s2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abc", "xya"), true),
            (("abe", "acd"), false),
            (("leetcodee", "interview"), true),
            (("szy", "cid"), true),
            (("abc", "cab"), true),
        ];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(S::check_if_can_break(s1.to_string(), s2.to_string()), expected);
        }
    }
}
