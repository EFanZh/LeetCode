pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::str;

impl Solution {
    fn encode(s: &str) -> u64 {
        let mut result = 0;

        for c in s.bytes() {
            result = (result << 5) | u64::from(c & 31);
        }

        result << (64 - s.len() * 5)
    }

    fn decode(mut v: u64, target: &mut String) {
        while v != 0 {
            target.push(char::from(0b_0110_0000 | (v >> 59) as u8));
            v <<= 5;
        }
    }

    pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
        let mut result = String::new();
        let mut dedup = HashSet::new();
        let mut counts = HashMap::new();

        for response in responses {
            dedup.extend(response.into_iter().map(|s| {
                let target = Self::encode(&s);

                if s.capacity() > result.capacity() {
                    result = s;
                }

                target
            }));

            for value in dedup.drain() {
                match counts.entry(value) {
                    Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(1_u32);
                    }
                }
            }
        }

        let encoded = counts
            .into_iter()
            .fold((0, 0), |(result, max_count), (k, v)| {
                if max_count < v {
                    (k, v)
                } else {
                    (if max_count == v { result.min(k) } else { result }, max_count)
                }
            })
            .0;

        result.clear();

        Self::decode(encoded, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_common_response(responses: Vec<Vec<String>>) -> String {
        Self::find_common_response(responses)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
