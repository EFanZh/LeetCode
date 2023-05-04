pub mod stack_updates;

pub trait SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self;
    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32);
    fn get_value(&self, row: i32, col: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::SubrectangleQueries;
    use crate::test_utilities::Matrix;

    enum Operation {
        UpdateSubrectangle(i32, i32, i32, i32, i32),
        GetValue((i32, i32), i32),
    }

    pub fn run<S: SubrectangleQueries>() {
        let test_cases = [
            (
                &[[1, 2, 1], [4, 3, 4], [3, 2, 1], [1, 1, 1]] as &dyn Matrix<_>,
                &[
                    Operation::GetValue((0, 2), 1),
                    Operation::UpdateSubrectangle(0, 0, 3, 2, 5),
                    Operation::GetValue((0, 2), 5),
                    Operation::GetValue((3, 1), 5),
                    Operation::UpdateSubrectangle(3, 0, 3, 2, 10),
                    Operation::GetValue((3, 1), 10),
                    Operation::GetValue((0, 2), 5),
                ] as &[_],
            ),
            (
                &[[1, 1, 1], [2, 2, 2], [3, 3, 3]],
                &[
                    Operation::GetValue((0, 0), 1),
                    Operation::UpdateSubrectangle(0, 0, 2, 2, 100),
                    Operation::GetValue((0, 0), 100),
                    Operation::GetValue((2, 2), 100),
                    Operation::UpdateSubrectangle(1, 1, 2, 2, 20),
                    Operation::GetValue((2, 2), 20),
                ],
            ),
            (
                &[[2, 8], [8, 8], [7, 4]],
                &[
                    Operation::GetValue((1, 0), 8),
                    Operation::UpdateSubrectangle(1, 1, 1, 1, 4),
                    Operation::GetValue((1, 0), 8),
                    Operation::GetValue((0, 0), 2),
                    Operation::UpdateSubrectangle(2, 1, 2, 1, 9),
                    Operation::GetValue((1, 1), 4),
                    Operation::GetValue((1, 0), 8),
                ],
            ),
        ];

        for (rectangle, operations) in test_cases {
            let mut subrectangle_queries = S::new(rectangle.to_vec());

            for operation in operations {
                match *operation {
                    Operation::UpdateSubrectangle(row1, col1, row2, col2, new_value) => {
                        subrectangle_queries.update_subrectangle(row1, col1, row2, col2, new_value);
                    }
                    Operation::GetValue((row, col), expected) => {
                        assert_eq!(subrectangle_queries.get_value(row, col), expected);
                    }
                }
            }
        }
    }
}
