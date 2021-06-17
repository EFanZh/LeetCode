pub mod iterator;
pub mod iterator_2;

pub trait Solution {
    fn reverse_vowels(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("hello", "holle"), ("leetcode", "leotcede")];

        for (s, expected) in test_cases {
            assert_eq!(S::reverse_vowels(s.to_string()), expected);
        }
    }
}
