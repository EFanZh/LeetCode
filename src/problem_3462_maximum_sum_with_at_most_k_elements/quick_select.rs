pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn keep_max(values: &mut Vec<u32>, n: usize) {
        if n < values.len() {
            values.select_nth_unstable_by(n, |lhs, rhs| rhs.cmp(lhs));
            values.truncate(n);
        }
    }

    pub fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
        let k = k.cast_unsigned() as usize;

        let mut values = grid
            .into_iter()
            .zip(limits)
            .flat_map(|(row, limit)| {
                let mut row = row.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();

                Self::keep_max(&mut row, limit.cast_unsigned() as _);

                row
            })
            .collect::<Vec<_>>();

        Self::keep_max(&mut values, k);

        values.iter().fold(0, |sum, &value| sum + i64::from(value))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
        Self::max_sum(grid, limits, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
