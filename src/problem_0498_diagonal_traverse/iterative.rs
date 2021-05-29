pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = mat.len();
        let columns = mat.first().map_or(0, Vec::len);
        let mut result = Vec::with_capacity(columns * rows);

        for sum in 0..(rows + columns - 1) {
            let iter = mat
                .iter()
                .enumerate()
                .take((sum + 1).min(rows))
                .skip(sum.saturating_sub(columns - 1))
                .map(|(i, row)| row[sum - i]);

            if sum % 2 == 0 {
                result.extend(iter.rev());
            } else {
                result.extend(iter);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_diagonal_order(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
