pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut available = HashMap::new();

        for letter in magazine.bytes() {
            available.entry(letter).and_modify(|count| *count += 1).or_insert(1);
        }

        for letter in ransom_note.bytes() {
            if let Entry::Occupied(entry) = available.entry(letter) {
                if *entry.get() == 1 {
                    entry.remove();
                } else {
                    *entry.into_mut() -= 1;
                }
            } else {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_construct(ransom_note: String, magazine: String) -> bool {
        Self::can_construct(ransom_note, magazine)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
