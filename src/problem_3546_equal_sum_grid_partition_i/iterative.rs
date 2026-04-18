pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let columns = grid.first().map_or(0, Vec::len);
        let mut buffer = Vec::with_capacity(grid.len().max(columns));
        let mut sum = 0;

        buffer.extend(grid.iter().map(|row| {
            for &value in row {
                sum += u64::from(value.cast_unsigned());
            }

            sum
        }));

        if !sum.is_multiple_of(2) {
            return false;
        }

        let half = sum / 2;

        if buffer.binary_search(&half).is_ok() {
            return true;
        }

        buffer.clear();

        sum = 0;

        buffer.extend((0..columns).map(|column| {
            for row in &grid {
                sum += u64::from(row[column].cast_unsigned());
            }

            sum
        }));

        buffer.binary_search(&half).is_ok()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        Self::can_partition_grid(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
