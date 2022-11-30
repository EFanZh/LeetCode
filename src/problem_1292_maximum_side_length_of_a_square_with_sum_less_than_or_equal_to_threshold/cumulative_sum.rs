pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let mut mat = mat;
        let threshold = threshold as u32;

        // Calculate cumulative sums.

        let mut iter = mat.iter_mut().map(Vec::as_mut_slice);

        let first_row = iter.next().unwrap();
        let mut sum = 0;

        for target in &mut *first_row {
            sum += *target;
            *target = sum;
        }

        let mut prev_row: &[_] = first_row;

        for row in iter {
            let mut row_sum = 0;

            for (target, &top) in row.iter_mut().zip(prev_row) {
                row_sum += *target;
                *target = top + row_sum;
            }

            prev_row = row;
        }

        // Calculate result.

        let mut result = 0;

        for (y, row) in mat.iter().enumerate() {
            for (x, &sum) in row.iter().enumerate() {
                if y >= result && x >= result {
                    let candidate = result + 1;

                    let (top_left, top) = mat.get(y.wrapping_sub(candidate)).map_or((0, 0), |top_row| {
                        (top_row.get(x.wrapping_sub(candidate)).copied().unwrap_or(0), top_row[x])
                    });

                    let left = row.get(x.wrapping_sub(candidate)).copied().unwrap_or(0);

                    if (sum + top_left - top - left) as u32 <= threshold {
                        result = candidate;
                    }
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        Self::max_side_length(mat, threshold)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
