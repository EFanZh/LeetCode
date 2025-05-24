pub mod mathematical;

pub trait Solution {
    fn count_anagrams(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("too hot", 18), ("aa", 1)];

        for (s, expected) in test_cases {
            assert_eq!(S::count_anagrams(s.to_string()), expected);
        }
    }
}
