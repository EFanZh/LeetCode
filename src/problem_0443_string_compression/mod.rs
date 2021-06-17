pub mod iterative;

pub trait Solution {
    fn compress(chars: &mut Vec<char>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("aabbccc", "a2b2c3"),
            ("a", "a"),
            ("abbbbbbbbbbbb", "ab12"),
            ("aaabbaa", "a3b2a2"),
            ("aaabaa", "a3ba2"),
            ("aaabba", "a3b2a"),
        ];

        for (s, expected) in test_cases {
            let mut s = s.chars().collect();
            let result_length = S::compress(&mut s);

            assert_eq!(s[..result_length as _].iter().copied().collect::<String>(), expected);
        }
    }
}
