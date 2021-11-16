pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

// <https://leetcode.com/problems/hand-of-straights/discuss/137794/Python-true-O(N)-solution>.

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize == 0 {
            let mut cards = HashMap::with_hasher(BuildHasherDefault::<DefaultHasher>::default());

            for card in hand {
                cards.entry(card).and_modify(|count| *count += 1).or_insert(1);
            }

            let mut straights = HashMap::new();
            let mut opened = 0;

            while let Some(&card) = cards.keys().next() {
                let mut card = card;

                while cards.contains_key(&(card - 1)) {
                    card -= 1;
                }

                while let Some(count) = cards.remove(&card) {
                    if count < opened {
                        return false;
                    }

                    let new_open = count - opened;

                    if new_open != 0 {
                        straights.insert(card, new_open);
                    }

                    opened = straights
                        .remove(&(card - group_size + 1))
                        .map_or(count, |closed| count - closed);

                    card += 1;
                }

                if opened != 0 {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        Self::is_n_straight_hand(hand, group_size)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
