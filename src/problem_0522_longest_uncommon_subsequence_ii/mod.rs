pub mod trie;

pub trait Solution {
    fn find_lu_slength(strs: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["aba", "cdc", "eae"] as &[_], 3),
            (&["aaa", "aaa", "aa"], -1),
            (&["aabbcc", "aabbcc", "bc", "bcc", "aabbccc"], 7),
            (&["aabbcc", "aabbcc", "cb"], 2),
            (&["aabbcc", "aabbcc", "b", "bc"], -1),
        ];

        for (strs, expected) in test_cases {
            assert_eq!(
                S::find_lu_slength(strs.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
