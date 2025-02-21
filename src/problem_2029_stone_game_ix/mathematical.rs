pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut counts = [0_u32; 3];

        for stone in stones {
            counts[usize::from(stone as u16 % 3)] += 1;
        }

        let [zero, one, two] = counts;
        let (x, y) = if two < one { (two, one) } else { (one, two) };

        if zero % 2 == 0 { x != 0 } else { y - x > 2 }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn stone_game_ix(stones: Vec<i32>) -> bool {
        Self::stone_game_ix(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
