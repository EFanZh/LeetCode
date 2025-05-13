pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn decode_position(position: u8) -> (i8, i8) {
        let position = position as i8;

        (position >> 4, position & 0b_0000_1111)
    }

    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        let mut buffer = [0_u8; 49];

        (0..).step_by(16).zip(grid).for_each(|(y, row)| {
            (y..).zip(row).for_each(|(x, value)| buffer[value as u32 as usize] = x);
        });

        let mut iter = buffer[..n * n].iter().copied().map(Self::decode_position);

        iter.next().is_none_or(|mut prev| {
            prev == (0, 0)
                && iter.all(|position| {
                    let diff = (position.0 - prev.0, position.1 - prev.1);

                    prev = position;

                    matches!(diff.0 * diff.1, -2 | 2)
                })
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        Self::check_valid_grid(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
