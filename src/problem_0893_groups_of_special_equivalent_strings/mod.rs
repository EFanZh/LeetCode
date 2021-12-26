pub mod hash_set;

pub trait Solution {
    fn num_special_equiv_groups(words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["abcd", "cdab", "cbad", "xyzz", "zzxy", "zzyx"] as &[_], 3),
            (&["abc", "acb", "bac", "bca", "cab", "cba"], 3),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::num_special_equiv_groups(words.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
