pub mod dynamic_programming;

pub trait Solution {
    fn max_vowels(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abciiidef", 3), 3), (("aeiou", 2), 2), (("leetcode", 3), 2)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::max_vowels(s.to_string(), k), expected);
        }
    }
}
