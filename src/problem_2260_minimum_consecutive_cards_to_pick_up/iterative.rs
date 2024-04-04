pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut prevs = HashMap::new();
        let mut result = u32::MAX - 1;

        for (i, value) in (0_u32..).zip(cards) {
            match prevs.entry(value) {
                Entry::Occupied(entry) => {
                    let prev = entry.into_mut();

                    result = result.min(i - *prev);

                    *prev = i;
                }
                Entry::Vacant(entry) => {
                    entry.insert(i);
                }
            }
        }

        (result + 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        Self::minimum_card_pickup(cards)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
