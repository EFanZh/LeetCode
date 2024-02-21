pub mod iterative;

pub trait Solution {
    fn divide_string(s: String, k: i32, fill: char) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcdefghi", 3, 'x'), &["abc", "def", "ghi"] as &[_]),
            (("abcdefghij", 3, 'x'), &["abc", "def", "ghi", "jxx"]),
        ];

        for ((s, k, fill), expected) in test_cases {
            assert_eq!(S::divide_string(s.to_string(), k, fill), expected);
        }
    }
}
