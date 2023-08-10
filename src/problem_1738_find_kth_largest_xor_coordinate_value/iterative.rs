pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut matrix = matrix;
        let rows = matrix.len();
        let columns = matrix.first().map_or(0, Vec::len);
        let mut values = Vec::with_capacity(columns * rows);
        let mut iter = matrix.iter_mut().map(Vec::as_mut_slice);
        let mut top_row = iter.next().unwrap();
        let mut xor = 0;

        for value in &mut *top_row {
            xor ^= *value;
            *value = xor;

            values.push(xor);
        }

        for row in iter {
            let mut top_left = 0;
            let mut left = 0;

            for (value, top) in row.iter_mut().zip(top_row) {
                *value ^= top_left ^ *top ^ left;

                top_left = *top;
                left = *value;

                values.push(*value);
            }

            top_row = row;
        }

        *values
            .select_nth_unstable_by_key(k as u32 as usize - 1, |&x| Reverse(x))
            .1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::kth_largest_value(matrix, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
