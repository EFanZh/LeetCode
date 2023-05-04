// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct SubrectangleQueries {
    operations: Vec<(u8, u8, u8, u8, i32)>,
    rectangle: Box<[i32]>,
    columns: usize,
}

impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        let columns = rectangle.first().map_or(0, Vec::len);

        Self {
            operations: Vec::new(),
            rectangle: rectangle.into_iter().flatten().collect(),
            columns,
        }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        self.operations
            .push((row1 as _, col1 as _, row2 as _, col2 as _, new_value));
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        let row = row as u8;
        let col = col as u8;

        for &(row1, col1, row2, col2, new_value) in self.operations.iter().rev() {
            if row >= row1 && row <= row2 && col >= col1 && col <= col2 {
                return new_value;
            }
        }

        self.rectangle[self.columns * usize::from(row) + usize::from(col)]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::SubrectangleQueries for SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self::new(rectangle)
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        self.update_subrectangle(row1, col1, row2, col2, new_value);
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.get_value(row, col)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SubrectangleQueries>();
    }
}
