pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut counts = HashMap::from([(0, (1, 0))]);
        let mut xor = 0;
        let mut result = 0;

        for (length, value) in (1..).zip(arr) {
            xor ^= value;

            match counts.entry(xor) {
                Entry::Occupied(entry) => {
                    let state = entry.into_mut();

                    result += (length - 1) * state.0 - state.1;

                    state.0 += 1;
                    state.1 += length;
                }
                Entry::Vacant(entry) => {
                    entry.insert((1, length));
                }
            };
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_triplets(arr: Vec<i32>) -> i32 {
        Self::count_triplets(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
