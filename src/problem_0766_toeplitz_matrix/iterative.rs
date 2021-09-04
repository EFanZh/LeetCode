pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        matrix
            .iter()
            .zip(&matrix[1..])
            .all(|(left, right)| left[..left.len() - 1] == right[1..])
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        Self::is_toeplitz_matrix(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
