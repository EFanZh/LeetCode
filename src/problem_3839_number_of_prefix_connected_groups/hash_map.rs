pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn prefix_connected(words: Vec<String>, k: i32) -> i32 {
        let k = k.cast_unsigned() as usize;
        let mut counts = HashMap::new();

        for word in &words {
            if let Some(prefix) = word.as_bytes().get(..k) {
                match counts.entry(prefix) {
                    Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() = true,
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(false);
                    }
                }
            }
        }

        counts.values().copied().map(i32::from).sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn prefix_connected(words: Vec<String>, k: i32) -> i32 {
        Self::prefix_connected(words, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
