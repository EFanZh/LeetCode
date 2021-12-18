pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for row in &grid {
            for &height in row {
                if height != 0 {
                    result += height * 4 + 2;
                }
            }

            for (left, right) in row.iter().zip(&row[1..]) {
                result -= left.min(right) * 2;
            }
        }

        for (top_row, bottom_rom) in grid.iter().zip(&grid[1..]) {
            for (top, bottom) in top_row.iter().zip(bottom_rom) {
                result -= top.min(bottom) * 2;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        Self::surface_area(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
