// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

pub struct Spreadsheet {
    data: HashMap<u32, i32>,
}

impl Spreadsheet {
    fn new(rows: i32) -> Self {
        _ = rows;

        Self { data: HashMap::new() }
    }

    fn parse_cell_inner(first: u8, rest: &[u8]) -> u32 {
        let column = u32::from(first - b'A');
        let mut row = 0;

        for &c in rest {
            row = row * 10 + u32::from(c - b'0');
        }

        26 * row + column
    }

    fn parse_cell(cell: String) -> u32 {
        let (&first, rest) = cell.as_bytes().split_first().unwrap();

        Self::parse_cell_inner(first, rest)
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.data.insert(Self::parse_cell(cell), value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.data.remove(&Self::parse_cell(cell));
    }

    fn parse_value(&self, value: &str) -> i32 {
        let (&first, rest) = value.as_bytes().split_first().unwrap();

        if first.is_ascii_uppercase() {
            self.data
                .get(&Self::parse_cell_inner(first, rest))
                .copied()
                .unwrap_or(0)
        } else {
            let mut result = i32::from(first - b'0');

            for &c in rest {
                result = result * 10 + i32::from(c - b'0');
            }

            result
        }
    }

    fn get_value(&self, formula: String) -> i32 {
        let (lhs, rhs) = formula[1..].split_once('+').unwrap();

        self.parse_value(lhs) + self.parse_value(rhs)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Spreadsheet for Spreadsheet {
    fn new(rows: i32) -> Self {
        Self::new(rows)
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.set_cell(cell, value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.reset_cell(cell);
    }

    fn get_value(&self, formula: String) -> i32 {
        self.get_value(formula)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Spreadsheet>();
    }
}
