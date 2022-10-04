pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let rows = board.len();
        let columns = board.first().map_or(0, String::len);
        let mut cache = vec![(0_u32, 0_u32); columns * rows];

        cache[0] = (0, 1);

        for (y, row) in board.into_iter().enumerate() {
            for (x, c) in row.into_bytes().into_iter().enumerate() {
                let digit = match c {
                    b'S' => 0,
                    b'E' | b'X' => continue,
                    _ => c - b'0',
                };

                let mut result = (0, 0);

                for (prev_y, prev_x) in [
                    (y.wrapping_sub(1), x.wrapping_sub(1)),
                    (y.wrapping_sub(1), x),
                    (y, x.wrapping_sub(1)),
                ] {
                    if prev_y < rows && prev_x < columns {
                        let (prev_max_sum, prev_paths) = cache[columns * prev_y + prev_x];

                        match prev_max_sum.cmp(&result.0) {
                            Ordering::Less => {}
                            Ordering::Equal => {
                                let paths = result.1 + prev_paths;

                                result.1 = paths.checked_sub(1_000_000_007).unwrap_or(paths);
                            }
                            Ordering::Greater => result = (prev_max_sum, prev_paths),
                        }
                    }
                }

                result.0 += u32::from(digit);

                cache[columns * y + x] = result;
            }
        }

        let mut result = *cache.last().unwrap();

        if result.1 == 0 {
            result.0 = 0;
        }

        vec![result.0 as _, result.1 as _]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        Self::paths_with_max_score(board)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
