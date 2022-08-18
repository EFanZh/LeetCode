pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let mut matrix = matrix;
        let mut sum_counts = HashMap::with_capacity(matrix.first().map_or(0, Vec::len));
        let mut result = 0;

        for i in 0..matrix.len() {
            let mut iter = matrix[i..].iter_mut().map(Vec::as_mut_slice);
            let buffer = iter.next().unwrap();
            let mut sum = 0;

            sum_counts.insert(0, 1);

            for value in buffer.iter() {
                sum += *value;

                if let Some(&count) = sum_counts.get(&(sum - target)) {
                    result += count;
                }

                sum_counts.entry(sum).and_modify(|count| *count += 1).or_insert(1);
            }

            sum_counts.clear();

            for row in iter {
                let mut sum = 0;

                sum_counts.insert(0, 1);

                for (top_sum, &mut value) in buffer.iter_mut().zip(row) {
                    *top_sum += value;
                    sum += *top_sum;

                    if let Some(&count) = sum_counts.get(&(sum - target)) {
                        result += count;
                    }

                    sum_counts.entry(sum).and_modify(|count| *count += 1).or_insert(1);
                }

                sum_counts.clear();
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        Self::num_submatrix_sum_target(matrix, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
