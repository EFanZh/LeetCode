pub mod iterative;

pub trait Solution {
    fn sort_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aaaabbbbcccc", "abccbaabccba"), ("rat", "art")];

        for (s, expected) in test_cases {
            assert_eq!(S::sort_string(s.to_string()), expected);
        }
    }
}
