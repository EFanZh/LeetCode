pub mod greedy;

pub trait Solution {
    fn smallest_palindrome(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("z", "z"), ("babab", "abbba"), ("daccad", "acddca")];

        for (s, expected) in test_cases {
            assert_eq!(S::smallest_palindrome(s.to_string()), expected);
        }
    }
}
