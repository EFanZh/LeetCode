pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for first in 0..n / 2 {
            let last = n - 1 - first;

            for i in first..last {
                let j = n - 1 - i;

                let temp = matrix[first][i];

                matrix[first][i] = matrix[j][first];
                matrix[j][first] = matrix[last][j];
                matrix[last][j] = matrix[i][last];
                matrix[i][last] = temp;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Self::rotate(matrix);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
