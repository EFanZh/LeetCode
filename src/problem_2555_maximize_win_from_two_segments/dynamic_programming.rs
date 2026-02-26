pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let prize_positions = prize_positions.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut cache = vec![0_u32; prize_positions.len()].into_boxed_slice();
        let k = k.cast_unsigned();
        let mut start = 0;
        let mut result = 0;
        let mut max_segment = 0;

        for (i, &prize_position) in prize_positions.iter().enumerate() {
            while prize_positions[start] + k < prize_position {
                start += 1;
            }

            let score = (i + 1 - start) as u32;

            result = result.max(cache.get(start.wrapping_sub(1)).copied().unwrap_or(0) + score);
            max_segment = max_segment.max(score);
            cache[i] = max_segment;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        Self::maximize_win(prize_positions, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
