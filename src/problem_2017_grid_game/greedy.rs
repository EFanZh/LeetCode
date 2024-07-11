pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let [first_row, second_row]: &[_; 2] = grid.as_slice().try_into().ok().unwrap();
        let first_row_sum = first_row.iter().fold(0_u64, |sum, &x| sum + u64::from(x as u32));
        let mut result = u64::MAX;
        let mut top_sum = 0_u64;
        let mut bottom_sum = 0_u64;

        for (&top, &bottom) in first_row.iter().zip(second_row) {
            top_sum += u64::from(top as u32);
            result = result.min(bottom_sum.max(first_row_sum - top_sum));
            bottom_sum += u64::from(bottom as u32);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        Self::grid_game(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
