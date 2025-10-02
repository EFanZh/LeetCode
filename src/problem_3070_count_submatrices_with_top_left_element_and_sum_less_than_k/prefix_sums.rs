pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut grid = grid
            .into_iter()
            .map(|row| row.into_iter().map(i32::cast_unsigned).collect())
            .collect::<Vec<_>>();

        let k = k.cast_unsigned();
        let mut iter = grid.iter_mut().map(Vec::as_mut_slice);
        let mut result = 0;
        let mut prev_row = &*iter.next().unwrap();
        let mut sum = 0;

        for &value in prev_row {
            sum += value;

            if sum <= k {
                result += 1;
            } else {
                break;
            }
        }

        for row in iter {
            let mut sum = 0;

            prev_row.iter().zip(&mut *row).try_for_each(|(&top, value)| {
                *value += top;
                sum += *value;

                (sum <= k).then(|| result += 1)
            });

            prev_row = row;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::count_submatrices(grid, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
