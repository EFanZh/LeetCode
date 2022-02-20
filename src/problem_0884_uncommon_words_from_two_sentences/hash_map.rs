pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut states = HashMap::new();

        for s in [s1.as_str(), s2.as_str()] {
            for word in s.split(' ') {
                states.entry(word).and_modify(|state| *state = true).or_insert(false);
            }
        }

        states
            .into_iter()
            .filter_map(|(key, value)| if value { None } else { Some(key.to_string()) })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        Self::uncommon_from_sentences(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
