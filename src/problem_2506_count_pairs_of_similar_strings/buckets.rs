pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut counts = HashMap::new();
        let mut result = 0;

        for word in words {
            let mut key = 0_u32;

            for c in word.into_bytes() {
                key |= 1 << (c - b'a');
            }

            match counts.entry(key) {
                Entry::Occupied(occupied_entry) => {
                    result += *occupied_entry.get();
                    *occupied_entry.into_mut() += 1;
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn similar_pairs(words: Vec<String>) -> i32 {
        Self::similar_pairs(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
