pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut values = alice_values
            .into_iter()
            .zip(bob_values)
            .map(|(alice_value, bob_value)| (alice_value + bob_value, alice_value))
            .collect::<Box<_>>();

        values.sort_unstable_by_key(|&(total_value, _)| Reverse(total_value));

        let mut is_odd = false;
        let mut alice_score = 0;
        let mut bob_score = 0;

        for &(total_value, alice_value) in &*values {
            if is_odd {
                bob_score += total_value - alice_value;
            } else {
                alice_score += alice_value;
            }

            is_odd = !is_odd;
        }

        alice_score.cmp(&bob_score) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        Self::stone_game_vi(alice_values, bob_values)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
