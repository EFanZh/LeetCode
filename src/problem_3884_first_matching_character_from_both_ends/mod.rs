pub mod iterative;

pub trait Solution {
    fn first_matching_index(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abcacbd", 1), ("abc", 1), ("abcdab", -1)];

        for (s, expected) in test_cases {
            assert_eq!(S::first_matching_index(s.to_string()), expected);
        }
    }
}
