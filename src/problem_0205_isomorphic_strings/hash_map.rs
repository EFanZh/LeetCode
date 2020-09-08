pub struct Solution;

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut mapping = HashMap::new();
        let mut used = HashSet::new();

        for (left, right) in s.bytes().zip(t.bytes()) {
            match mapping.entry(left) {
                Entry::Occupied(entry) => {
                    if *entry.into_mut() != right {
                        return false;
                    }
                }
                Entry::Vacant(entry) => {
                    if !used.insert(right) {
                        return false;
                    }

                    entry.insert(right);
                }
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn is_isomorphic(s: String, t: String) -> bool {
        Self::is_isomorphic(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
