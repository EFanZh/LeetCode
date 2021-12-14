pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut result = 0;
        let mut base = 1;

        for column in (0..columns).rev() {
            let same = grid.iter().filter(|row| row[column] == row[0]).count();

            result += base * same.max(rows - same);

            base *= 2;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        Self::matrix_score(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
