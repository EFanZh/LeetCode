pub struct Solution;

impl Solution {
    fn dfs(matrix: &[Vec<i32>], i: usize, j: usize, columns: usize, cache: &mut [i32]) -> i32 {
        let key = columns * i + j;

        match cache[key] {
            0 => {
                let value = matrix[i][j];
                let mut max_length = 0;
                let row = matrix[i].as_slice();

                for (next, (next_i, next_j)) in [
                    matrix
                        .get(i.wrapping_sub(1))
                        .map(|row| (row[j], (i.wrapping_sub(1), j))),
                    row.get(j.wrapping_sub(1)).map(|&next| (next, (i, j.wrapping_sub(1)))),
                    row.get(j + 1).map(|&next| (next, (i, j + 1))),
                    matrix.get(i + 1).map(|row| (row[j], (i + 1, j))),
                ]
                .iter()
                .copied()
                .flatten()
                {
                    if value < next {
                        max_length = max_length.max(Self::dfs(matrix, next_i, next_j, columns, cache));
                    }
                }

                max_length += 1;

                cache[key] = max_length;

                max_length
            }
            length => length,
        }
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let rows = matrix.len();
        let columns = matrix.first().map_or(0, Vec::len);
        let mut cache = vec![0; columns * matrix.len()];

        for i in 0..rows {
            for j in 0..columns {
                result = result.max(Self::dfs(&matrix, i, j, columns, &mut cache));
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        Self::longest_increasing_path(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
