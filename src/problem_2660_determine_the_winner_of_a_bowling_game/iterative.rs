pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn calculate_score(player: &[i32]) -> u32 {
        let mut state = 0_u8;

        player
            .iter()
            .map(|&x| {
                let score = x as u32 * (1 + u32::from(state & 3 != 0));

                state = (state << 1) | u8::from(x == 10);

                score
            })
            .sum()
    }

    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        match u32::cmp(&Self::calculate_score(&player1), &Self::calculate_score(&player2)) {
            Ordering::Less => 2,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        Self::is_winner(player1, player2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
