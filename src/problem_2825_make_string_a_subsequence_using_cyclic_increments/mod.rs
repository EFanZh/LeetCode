pub mod greedy;

pub trait Solution {
    fn can_make_subsequence(str1: String, str2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abc", "ad"), true), (("zc", "ad"), true), (("ab", "d"), false)];

        for ((str1, str2), expected) in test_cases {
            assert_eq!(S::can_make_subsequence(str1.to_string(), str2.to_string()), expected);
        }
    }
}
