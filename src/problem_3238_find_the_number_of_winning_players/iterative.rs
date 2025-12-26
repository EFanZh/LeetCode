pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        const COLORS: usize = 11;

        let mut counts = [0_u8; COLORS * 10];

        for pick in pick {
            let [player, color] = pick.try_into().ok().unwrap();

            counts[player.cast_unsigned() as usize * COLORS + color.cast_unsigned() as usize] += 1;
        }

        (0..n as _)
            .zip(counts.as_chunks::<COLORS>().0)
            .filter(|&(player, counts)| counts.iter().any(|&count| count > player))
            .count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        Self::winning_player_count(n, pick)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
