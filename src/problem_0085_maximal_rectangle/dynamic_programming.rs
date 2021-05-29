pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let columns = matrix.first().map_or(0, Vec::len) as i32;
        let mut cache = vec![(0, columns, 0); columns as _];
        let mut result = 0;

        for row in matrix {
            let mut left_boundary = 0;

            for (i, ((left, _, height), c)) in (0..).zip(cache.iter_mut().zip(&row)) {
                if *c == '0' {
                    *left = 0;
                    *height = 0;
                    left_boundary = i + 1;
                } else {
                    *left = (*left).max(left_boundary);
                    *height += 1;
                }
            }

            let mut right_boundary = columns;

            for (i, ((left, right, height), c)) in (0..columns).zip(cache.iter_mut().zip(row)).rev() {
                if c == '0' {
                    *right = columns;
                    right_boundary = i;
                } else {
                    *right = (*right).min(right_boundary);
                    result = result.max((*right - *left) * *height);
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        Self::maximal_rectangle(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
