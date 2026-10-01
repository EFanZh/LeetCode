pub mod hash_map;

pub trait Spreadsheet {
    fn new(rows: i32) -> Self;
    fn set_cell(&mut self, cell: String, value: i32);
    fn reset_cell(&mut self, cell: String);
    fn get_value(&self, formula: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Spreadsheet;

    enum Operation {
        SetCell(&'static str, i32),
        ResetCell(&'static str),
        GetValue(&'static str, i32),
    }

    pub fn run<S: Spreadsheet>() {
        let test_cases = [
            (
                3,
                &[
                    Operation::GetValue("=5+7", 12),
                    Operation::SetCell("A1", 10),
                    Operation::GetValue("=A1+6", 16),
                    Operation::SetCell("B2", 15),
                    Operation::GetValue("=A1+B2", 25),
                    Operation::ResetCell("A1"),
                    Operation::GetValue("=A1+B2", 15),
                ] as &[_],
            ),
            (458, &[Operation::GetValue("=O126+10272", 10272)]),
        ];

        for (rows, operations) in test_cases {
            let mut spreadsheet = S::new(rows);

            for operation in operations {
                match *operation {
                    Operation::SetCell(cell, value) => spreadsheet.set_cell(cell.to_string(), value),
                    Operation::ResetCell(cell) => spreadsheet.reset_cell(cell.to_string()),
                    Operation::GetValue(formula, expected) => {
                        assert_eq!(spreadsheet.get_value(formula.to_string()), expected);
                    }
                }
            }
        }
    }
}
