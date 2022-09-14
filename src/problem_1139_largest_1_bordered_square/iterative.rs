pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut top_lengths = vec![0; columns];
        let mut max_length = 0;

        for y in 0..rows {
            let mut left_length = 0;

            for x in 0..columns {
                if grid[y][x] == 0 {
                    left_length = 0;
                    top_lengths[x] = 0;
                } else {
                    left_length += 1;
                    top_lengths[x] += 1;
                }

                grid[y][x] = left_length as _;

                for offset in (max_length..left_length.min(top_lengths[x])).rev() {
                    if top_lengths[x - offset] > offset && grid[y - offset][x] as usize > offset {
                        max_length = offset + 1;

                        break;
                    }
                }
            }
        }

        (max_length * max_length) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        Self::largest1_bordered_square(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
