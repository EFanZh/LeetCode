struct NumMatrix {
    sums: Box<[i32]>,
    columns: usize,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let columns = matrix.first().map_or(0, Vec::len);
        let mut sums = Vec::with_capacity(columns * matrix.len());

        for row in matrix {
            let mut row_sum = 0;

            for cell in row {
                row_sum += cell;

                sums.push(sums.get(sums.len().wrapping_sub(columns)).copied().unwrap_or(0) + row_sum);
            }
        }

        Self {
            sums: sums.into_boxed_slice(),
            columns,
        }
    }

    fn get_sum(&self, row: usize, column: usize) -> i32 {
        self.sums
            .get(self.columns * row..)
            .and_then(|row| row.get(column).copied())
            .unwrap_or(0)
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = (row1 as usize).wrapping_sub(1);
        let col1 = (col1 as usize).wrapping_sub(1);
        let row2 = row2 as usize;
        let col2 = col2 as usize;

        self.get_sum(row2, col2) + self.get_sum(row1, col1) - (self.get_sum(row1, col2) + self.get_sum(row2, col1))
    }
}

impl super::NumMatrix for NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        Self::new(matrix)
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.sum_region(row1, col1, row2, col2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::NumMatrix>();
    }
}
