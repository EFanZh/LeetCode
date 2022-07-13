pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let m = m as u32;
        let n = n as u32;
        let mut row_states = 0_u64;
        let mut column_states = 0_u64;

        for index in &indices {
            let [r, c]: [_; 2] = index.as_slice().try_into().unwrap();

            row_states ^= 1 << r;
            column_states ^= 1 << c;
        }

        let row_flipped = row_states.count_ones();
        let column_flipped = column_states.count_ones();

        (row_flipped * n + column_flipped * m - row_flipped * column_flipped * 2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        Self::odd_cells(m, n, indices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
