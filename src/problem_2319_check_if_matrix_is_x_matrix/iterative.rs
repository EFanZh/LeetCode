pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        let n_minus_1 = n - 1;

        grid.iter().enumerate().all(|(y, row)| {
            let other_x = n_minus_1 - y;

            row.iter()
                .enumerate()
                .all(|(x, &value)| (value != 0) == (x == y || x == other_x))
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        Self::check_x_matrix(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
