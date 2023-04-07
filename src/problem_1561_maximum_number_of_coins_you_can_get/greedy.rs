pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles = piles;
        let n = piles.len();
        let one_third = n / 3;

        piles.select_nth_unstable(one_third);

        let right = &mut piles[one_third..];

        right.sort_unstable();

        right.iter().step_by(2).sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_coins(piles: Vec<i32>) -> i32 {
        Self::max_coins(piles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
