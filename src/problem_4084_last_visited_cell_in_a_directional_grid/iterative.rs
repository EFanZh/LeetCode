pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn last_visited_cell(grid: Vec<Vec<char>>) -> Vec<i32> {
        let mut grid = grid;
        let mut current_row = 0_usize;
        let mut current_column = 0_usize;
        let mut cell = &mut grid[0][0];

        loop {
            let (next_row, next_column) = match cell {
                'D' => (current_row + 1, current_column),
                'L' => (current_row, current_column.wrapping_sub(1)),
                'R' => (current_row, current_column + 1),
                _ => (current_row.wrapping_sub(1), current_column),
            };

            *cell = '\0';

            if let Some(next_cell) = grid
                .get_mut(next_row)
                .and_then(|row| row.get_mut(next_column))
                .filter(|next_cell| **next_cell != '\0')
            {
                current_row = next_row;
                current_column = next_column;
                cell = next_cell;
            } else {
                break;
            }
        }

        vec![current_row as _, current_column as _]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn last_visited_cell(grid: Vec<Vec<char>>) -> Vec<i32> {
        Self::last_visited_cell(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
