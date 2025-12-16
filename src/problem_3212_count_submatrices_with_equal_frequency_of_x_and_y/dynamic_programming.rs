pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        const X: u32 = 'X' as _;
        const Y: u32 = 'Y' as _;

        let mut iter = grid
            .into_iter()
            .map(|row| row.into_iter().map(u32::from).collect::<Vec<_>>());

        iter.next().map_or(0, |mut cache| {
            let mut result = 0;
            let mut total_x = 0;
            let mut total_y = 0;

            for state in &mut cache {
                *state = match *state {
                    X => 1 << 16,
                    Y => 1,
                    _ => 0,
                };

                total_x += *state >> 16;
                total_y += *state & 0x_ffff;
                result += i32::from(total_x == total_y && total_x != 0);
            }

            for row in iter {
                total_x = 0;
                total_y = 0;

                cache.iter_mut().zip(row).for_each(|(state, value)| {
                    *state += match value {
                        X => 1 << 16,
                        Y => 1,
                        _ => 0,
                    };

                    total_x += *state >> 16;
                    total_y += *state & 0x_ffff;
                    result += i32::from(total_x == total_y && total_x != 0);
                });
            }

            result
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        Self::number_of_submatrices(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
