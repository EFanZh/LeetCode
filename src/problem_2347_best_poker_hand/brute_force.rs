pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(ranks: &[i32; 5], suits: &[char; 5]) -> &'static str {
        let first_suit = suits[0];

        if suits[1..].iter().all(|&suit| suit == first_suit) {
            return "Flush";
        }

        let mut counts = [0_u8; 13];
        let mut ranks_iter = ranks.iter().copied();

        while let Some(rank) = ranks_iter.next() {
            let count = &mut counts[(rank as usize).wrapping_sub(1)];

            *count += 1;

            if *count == 2 {
                for rank in ranks_iter {
                    let count = &mut counts[(rank as usize).wrapping_sub(1)];

                    if *count == 2 {
                        return "Three of a Kind";
                    }

                    *count += 1;
                }

                return "Pair";
            }
        }

        "High Card"
    }

    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let (ranks, suits) = Option::zip(ranks.as_slice().try_into().ok(), suits.as_slice().try_into().ok()).unwrap();

        Self::helper(ranks, suits).to_string()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        Self::best_hand(ranks, suits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
