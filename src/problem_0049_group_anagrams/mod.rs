pub mod sort_word;

pub trait Solution {
    fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &["eat", "tea", "tan", "ate", "nat", "bat"] as &[_],
            &[&["ate", "eat", "tea"] as &[_], &["bat"], &["nat", "tan"]] as &[_],
        )];

        for (strs, expected) in test_cases {
            let strs = strs.iter().copied().map(str::to_string).collect();

            assert_eq!(
                test_utilities::unstable_sorted(
                    S::group_anagrams(strs).into_iter().map(test_utilities::unstable_sorted)
                ),
                expected
            );
        }
    }
}
