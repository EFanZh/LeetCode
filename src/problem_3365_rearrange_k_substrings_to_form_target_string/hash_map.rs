pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let n = s.len() / k.cast_unsigned() as usize;
        let mut counts = HashMap::new();

        for chunk in s.as_bytes().chunks_exact(n) {
            match counts.entry(chunk) {
                Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(0);
                }
            }
        }

        for chunk in t.as_bytes().chunks_exact(n) {
            let Entry::Occupied(occupied_entry) = counts.entry(chunk) else {
                return false;
            };

            if *occupied_entry.get() == 0 {
                occupied_entry.remove();
            } else {
                *occupied_entry.into_mut() -= 1;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        Self::is_possible_to_rearrange(s, t, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
