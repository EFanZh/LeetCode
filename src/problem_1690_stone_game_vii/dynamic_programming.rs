pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        let n = stones.len();
        let mut cache = vec![0; n].into_boxed_slice();
        let mut sum = 0;

        for stone in &mut stones {
            sum += *stone;
            *stone = sum;
        }

        for length in 2..=n {
            for start in 0..=n - length {
                let left_sum = stones[start + length - 2] - stones.get(start.wrapping_sub(1)).copied().unwrap_or(0);
                let right_sum = stones[start + length - 1] - stones[start];

                cache[start] = (left_sum - cache[start]).max(right_sum - cache[start + 1]);
            }
        }

        cache[0]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn stone_game_vii(stones: Vec<i32>) -> i32 {
        Self::stone_game_vii(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
