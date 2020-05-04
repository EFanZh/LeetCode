pub struct Solution {}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut buckets = HashMap::new();

        for str in strs {
            let mut key = str.as_bytes().to_vec();

            key.sort_unstable();

            match buckets.entry(key) {
                Entry::Vacant(entry) => {
                    entry.insert(vec![str]);
                }
                Entry::Occupied(entry) => {
                    entry.into_mut().push(str);
                }
            }
        }

        buckets.into_iter().map(|(_, value)| value).collect()
    }
}

impl super::Solution for Solution {
    fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        Self::group_anagrams(strs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
