pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let columns = matrix.first().map_or(0, Vec::len);

        (0..columns)
            .map(|i| matrix.iter().map(|row| row[i]).collect())
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::transpose(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
