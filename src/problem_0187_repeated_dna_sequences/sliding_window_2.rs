pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    fn get_digit(c: u8) -> u32 {
        match c {
            b'A' => 0,
            b'C' => 1,
            b'G' => 2,
            _ => 3,
        }
    }

    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut result = Vec::new();

        if s.len() > 10 {
            let mut counts = HashMap::new();
            let mut key = 0;
            let (first, rest) = s.split_at(9);

            for c in first.bytes() {
                key <<= 2;
                key |= Self::get_digit(c);
            }

            for (i, c) in rest.bytes().enumerate() {
                key &= (1 << 18) - 1;
                key <<= 2;
                key |= Self::get_digit(c);

                match counts.entry(key) {
                    Entry::Vacant(entry) => {
                        entry.insert(false);
                    }
                    Entry::Occupied(entry) => {
                        let count = entry.into_mut();

                        if !*count {
                            *count = true;

                            result.push(s[i..i + 10].to_string());
                        }
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
