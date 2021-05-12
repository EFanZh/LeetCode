pub struct Solution;

// TODO: Unify forward and backward steps.

impl Solution {
    fn update_forward(mat: &mut [Vec<i32>]) {
        let mut rows_iter = mat.iter_mut().map(Vec::as_mut_slice);
        let mut prev_row = rows_iter.next().unwrap();
        let mut first_row_iter = prev_row.iter_mut();
        let mut left = *first_row_iter.next().unwrap();

        for value in first_row_iter {
            *value = (*value).min(left + 1);
            left = *value;
        }

        for row in rows_iter {
            let mut iter = prev_row.iter().copied().zip(row.iter_mut());
            let (top, left) = iter.next().unwrap();

            *left = (*left).min(top + 1);

            let mut left = *left;

            for (top, current) in iter {
                *current = (*current).min(top.min(left) + 1);
                left = *current;
            }

            prev_row = row;
        }
    }

    fn update_backward(mat: &mut [Vec<i32>]) {
        let mut rows_iter = mat.iter_mut().map(Vec::as_mut_slice).rev();
        let mut prev_row = rows_iter.next().unwrap();
        let mut first_row_iter = prev_row.iter_mut().rev();
        let mut left = *first_row_iter.next().unwrap();

        for value in first_row_iter {
            *value = (*value).min(left + 1);
            left = *value;
        }

        for row in rows_iter {
            let mut iter = prev_row.iter().rev().copied().zip(row.iter_mut().rev());
            let (top, left) = iter.next().unwrap();

            *left = (*left).min(top + 1);

            let mut left = *left;

            for (top, current) in iter {
                *current = (*current).min(top.min(left) + 1);
                left = *current;
            }

            prev_row = row;
        }
    }

    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in &mut mat {
            for cell in row {
                if *cell != 0 {
                    *cell = i32::max_value() - 1;
                }
            }
        }

        Self::update_forward(&mut mat);
        Self::update_backward(&mut mat);

        mat
    }
}

impl super::Solution for Solution {
    fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::update_matrix(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
