pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;

        if hand.len() % group_size == 0 {
            let mut cards = HashMap::new();

            for card in hand {
                cards.entry(card).and_modify(|count| *count += 1).or_insert(1);
            }

            let mut cards = cards.into_iter().collect::<Vec<_>>();

            cards.sort_unstable_by_key(|&(card, _)| card);

            if let Some(max_i) = cards.len().checked_sub(group_size) {
                for i in 0..=max_i {
                    let (&mut (first_card, first_count), rest) = cards[i..i + group_size].split_first_mut().unwrap();

                    if first_count != 0 {
                        let mut prev_card = first_card;

                        for (card, count) in rest {
                            if *card == prev_card + 1 && *count >= first_count {
                                *count -= first_count;
                                prev_card = *card;
                            } else {
                                return false;
                            }
                        }
                    }
                }

                return cards[cards.len() - group_size + 1..]
                    .iter()
                    .all(|&(_, count)| count == 0);
            }
        }

        false
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
