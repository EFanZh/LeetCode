pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let k = k as usize;
        let rows = mat.len();
        let columns = mat.first().map_or(0, Vec::len);
        let mut iter = mat.iter_mut().map(Vec::as_mut_slice);
        let first_row = iter.next().unwrap();
        let mut sum = 0;

        for target in &mut *first_row {
            sum += *target;
            *target = sum;
        }

        let mut prev_row = &*first_row;

        for row in iter {
            let mut sum = 0;

            for (target, &prev_sum) in row.iter_mut().zip(prev_row) {
                sum += *target;
                *target = sum + prev_sum;
            }

            prev_row = row;
        }

        let last_row = mat.last().unwrap().as_slice();

        let get_sum = |y: usize, x: usize| {
            let row = if let Some(row) = mat.get(y) {
                row.as_slice()
            } else if y <= usize::MAX / 2 {
                last_row
            } else {
                return 0;
            };

            if let Some(&value) = row.get(x) {
                value
            } else if x <= usize::MAX / 2 {
                *row.last().unwrap()
            } else {
                0
            }
        };

        (0..rows)
            .map(|y| {
                (0..columns)
                    .map(|x| {
                        let y_1 = y.wrapping_sub(k + 1);
                        let y_2 = y + k;
                        let x_1 = x.wrapping_sub(k + 1);
                        let x_2 = x + k;

                        get_sum(y_2, x_2) + get_sum(y_1, x_1) - get_sum(y_1, x_2) - get_sum(y_2, x_1)
                    })
                    .collect()
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        Self::matrix_block_sum(mat, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
