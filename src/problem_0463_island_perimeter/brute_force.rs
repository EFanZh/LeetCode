pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell != 0 {
                    if grid.get(i.wrapping_sub(1)).is_none_or(|r| r[j] == 0) {
                        result += 1;
                    }

                    if row.get(j.wrapping_sub(1)).is_none_or(|&c| c == 0) {
                        result += 1;
                    }

                    if row.get(j + 1).is_none_or(|&c| c == 0) {
                        result += 1;
                    }

                    if grid.get(i + 1).is_none_or(|r| r[j] == 0) {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        Self::island_perimeter(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
