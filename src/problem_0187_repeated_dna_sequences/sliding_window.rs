pub struct Solution {}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut counts = HashMap::new();

        for slice in s.as_bytes().windows(10) {
            match counts.entry(slice) {
                Entry::Vacant(entry) => {
                    entry.insert(false);
                }
                Entry::Occupied(entry) => {
                    let count = entry.into_mut();

                    if !*count {
                        *count = true;

                        result.push(String::from_utf8(slice.to_vec()).unwrap());
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        Self::find_repeated_dna_sequences(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
