pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn make_iter(grid: &mut [Vec<i32>], mut start_column: usize) -> impl Iterator<Item = &mut i32> {
        grid.iter_mut()
            .map_while(move |row| row.get_mut(start_column).inspect(|_| start_column += 1))
    }

    fn process(grid: &mut [Vec<i32>], start_column: usize) {
        let mut right_counts = [0_u8; 51];
        let mut unique_right_count = 0;

        for value in Self::make_iter(grid, start_column) {
            let count = &mut right_counts[value.cast_unsigned() as usize];

            unique_right_count += u32::from(*count == 0);
            *count += 1;
        }

        let mut left_counts = [0_u8; 51];
        let mut unique_left_count = 0;

        for target in Self::make_iter(grid, start_column) {
            let value = target.cast_unsigned() as usize;
            let right_count = &mut right_counts[value];

            *right_count -= 1;
            unique_right_count -= u32::from(*right_count == 0);

            *target = u32::abs_diff(unique_left_count, unique_right_count).cast_signed();

            let left_count = &mut left_counts[value];

            unique_left_count += u32::from(*left_count == 0);
            *left_count += 1;
        }
    }

    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let columns = grid.first().map_or(0, Vec::len);

        for start_column in 0..columns {
            Self::process(&mut grid, start_column);
        }

        for y in 1..grid.len() {
            Self::process(&mut grid[y..], 0);
        }

        grid
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::difference_of_distinct_values(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
