pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut matrix = matrix;
        let columns = matrix.first().map_or(0, Vec::len);

        (0..columns).for_each(|column| {
            let max = matrix.iter().fold(i32::MIN, |max, row| max.max(row[column]));

            for row in &mut matrix {
                let value = &mut row[column];

                if *value == -1 {
                    *value = max;
                }
            }
        });

        matrix
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::modified_matrix(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
