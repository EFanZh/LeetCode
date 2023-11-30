pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut candidate_sum = 0_i64;
        let mut min_abs = i32::MAX;
        let mut has_odd_negatives = false;

        for row in matrix {
            for mut value in row {
                if value < 0 {
                    value = -value;
                    has_odd_negatives = !has_odd_negatives;
                }

                candidate_sum += i64::from(value);
                min_abs = min_abs.min(value);
            }
        }

        if has_odd_negatives {
            candidate_sum -= i64::from(min_abs) * 2;
        }

        candidate_sum
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        Self::max_matrix_sum(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
