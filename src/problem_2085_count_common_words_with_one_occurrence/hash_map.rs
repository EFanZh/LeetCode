pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut states = HashMap::<_, u8>::new();

        for word in words1 {
            states.entry(word).and_modify(|c| *c = 1).or_insert(0);
        }

        let mut result = 0;

        for word in words2 {
            if let Entry::Occupied(mut entry) = states.entry(word) {
                let state = entry.get_mut();

                if *state == 0 {
                    result += 1;
                    *state = 2;
                } else {
                    if *state == 2 {
                        result -= 1;
                    }

                    entry.remove();
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        Self::count_words(words1, words2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
