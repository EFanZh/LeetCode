pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        // Top.

        for row in &grid {
            for &cell in row {
                if cell != 0 {
                    result += 1;
                }
            }
        }

        // Front.

        for i in 0..grid.first().map_or(0, Vec::len) {
            let mut max = 0;

            for row in &grid {
                max = max.max(row[i]);
            }

            result += max;
        }

        // Side.

        for row in &grid {
            let mut max = 0;

            for &cell in row {
                max = max.max(cell);
            }

            result += max;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        Self::projection_area(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
