pub mod iterative;

pub trait Solution {
    fn majority_frequency_group(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aaabbbccdddde", "ab"), ("abcd", "abcd"), ("pfpfgi", "fp")];

        for (s, expected) in test_cases {
            assert_eq!(S::majority_frequency_group(s.to_string()), expected);
        }
    }
}
