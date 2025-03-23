pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;

        for row in &mut grid {
            row.sort_unstable();
        }

        let columns = grid.first().map_or(0, Vec::len);

        (0..columns).fold(0, |sum, i| sum + grid.iter().fold(i32::MIN, |max, row| max.max(row[i])))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        Self::delete_greatest_value(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
