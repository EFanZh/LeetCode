pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        if (grid.len() + grid.first().map_or(0, Vec::len)).is_multiple_of(2) {
            false
        } else {
            let mut state = [0_u128; 100];

            state[0] = 1;

            let mut left = 0;

            for row in grid {
                left = 0;

                for (top, c) in state.iter_mut().zip(row) {
                    *top |= left;

                    if c == '(' {
                        *top <<= 1;
                    } else {
                        *top >>= 1;
                    }

                    left = *top;
                }
            }

            left & 1 != 0
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        Self::has_valid_path(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
