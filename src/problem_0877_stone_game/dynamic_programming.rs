pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut cache = piles.clone();

        for length in 2..=n {
            for start in 0..=n - length {
                cache[start] = (piles[start + length - 1] - cache[start]).max(piles[start] - cache[start + 1]);
            }
        }

        piles[0] > 0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn stone_game(piles: Vec<i32>) -> bool {
        Self::stone_game(piles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
