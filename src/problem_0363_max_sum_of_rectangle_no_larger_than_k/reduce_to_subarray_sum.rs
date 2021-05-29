pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BTreeSet;
use std::iter;

impl Solution {
    fn helper(values: impl Iterator<Item = i32>, sums: &mut [i32], k: i32, result: &mut i32) -> bool {
        let mut prefix_sum = 0;
        let mut min_prefix_sum = 0;
        let mut max_prefix_sum = 0;
        let mut min_range_sum = i32::max_value();
        let mut max_range_sum = i32::min_value();

        for (value, sum) in values.zip(sums.iter_mut()) {
            *sum += value;
            prefix_sum += *sum;
            min_range_sum = min_range_sum.min(prefix_sum - max_prefix_sum);
            max_range_sum = max_range_sum.max(prefix_sum - min_prefix_sum);
            min_prefix_sum = min_prefix_sum.min(prefix_sum);
            max_prefix_sum = max_prefix_sum.max(prefix_sum);
        }

        if min_range_sum == k || max_range_sum == k {
            return true;
        }

        if max_range_sum < k {
            (*result) = (*result).max(max_range_sum);
        } else if min_range_sum < k {
            let mut prefix_sums = iter::once(0).collect::<BTreeSet<_>>();

            prefix_sum = 0;

            for sum in sums.iter() {
                prefix_sum += sum;

                let min_prefix_sum = prefix_sum - k;

                if let Some(&prev_prefix_sum) = prefix_sums.range(min_prefix_sum..).next() {
                    if prev_prefix_sum == min_prefix_sum {
                        return true;
                    }

                    (*result) = (*result).max(prefix_sum - prev_prefix_sum);
                }

                prefix_sums.insert(prefix_sum);
            }
        }

        false
    }

    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = matrix.len();
        let columns = matrix.first().map_or(0, Vec::len);
        let mut result = i32::min_value();

        if columns < rows {
            let mut row_sums = vec![0; rows];

            for left in 0..columns {
                for column in left..columns {
                    if Self::helper(matrix.iter().map(|row| row[column]), &mut row_sums, k, &mut result) {
                        return k;
                    }
                }

                row_sums.iter_mut().for_each(|x| *x = 0);
            }
        } else {
            let mut column_sums = vec![0; columns];

            for top in 0..rows {
                for row in &matrix[top..] {
                    if Self::helper(row.iter().copied(), &mut column_sums, k, &mut result) {
                        return k;
                    }
                }

                column_sums.iter_mut().for_each(|x| *x = 0);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::max_sum_submatrix(matrix, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
