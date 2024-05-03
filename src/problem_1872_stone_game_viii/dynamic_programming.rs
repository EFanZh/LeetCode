pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        let stones = stones.as_mut_slice();
        let n = stones.len();

        assert!(n > 1);

        let mut sum = 0;

        for target in &mut *stones {
            sum += *target;
            *target = sum;
        }

        let mut diff = sum;

        for &value in stones[1..n - 1].iter().rev() {
            diff = diff.max(value - diff);
        }

        diff
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn stone_game_viii(stones: Vec<i32>) -> i32 {
        Self::stone_game_viii(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
