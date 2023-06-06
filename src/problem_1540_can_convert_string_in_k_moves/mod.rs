pub mod buckets;

pub trait Solution {
    fn can_convert_string(s: String, t: String, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("input", "ouput", 9), true),
            (("abc", "bcd", 10), false),
            (("aab", "bbb", 27), true),
            (("abc", "abcd", 1000), false),
            (("iqssxdlb", "dyuqrwyr", 40), true),
        ];

        for ((s, t, k), expected) in test_cases {
            assert_eq!(S::can_convert_string(s.to_string(), t.to_string(), k), expected);
        }
    }
}
