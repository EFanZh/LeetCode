pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let (first_row, rest_rows) = grid.split_first().unwrap();
        let mut prev = -1;

        first_row.iter().all(|&value| {
            let result = value != prev;

            prev = value;

            result
        }) && first_row
            .iter()
            .enumerate()
            .all(|(column, &value)| rest_rows.iter().all(|row| row[column] == value))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        Self::satisfies_conditions(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
