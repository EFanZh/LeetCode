pub mod precomputed;

pub trait NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self;
    fn adjacent_sum(&self, value: i32) -> i32;
    fn diagonal_sum(&self, value: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::NeighborSum;
    use crate::test_utilities::Matrix;

    enum Operation {
        AdjacentSum(i32, i32),
        DiagonalSum(i32, i32),
    }

    pub fn run<N: NeighborSum>() {
        let test_cases = [
            (
                &[[0, 1, 2], [3, 4, 5], [6, 7, 8]] as &dyn Matrix<_>,
                &[
                    Operation::AdjacentSum(1, 6),
                    Operation::AdjacentSum(4, 16),
                    Operation::AdjacentSum(4, 16),
                    Operation::DiagonalSum(4, 16),
                    Operation::DiagonalSum(8, 4),
                ] as &[_],
            ),
            (
                &[[1, 2, 0, 3], [4, 7, 15, 6], [8, 9, 10, 11], [12, 13, 14, 5]],
                &[Operation::AdjacentSum(15, 23), Operation::DiagonalSum(9, 45)],
            ),
            (
                &[[3, 6, 0], [8, 5, 1], [2, 4, 7]],
                &[
                    Operation::AdjacentSum(1, 12),
                    Operation::AdjacentSum(3, 14),
                    Operation::AdjacentSum(7, 5),
                    Operation::AdjacentSum(2, 12),
                    Operation::AdjacentSum(5, 19),
                    Operation::AdjacentSum(8, 10),
                    Operation::AdjacentSum(0, 7),
                    Operation::DiagonalSum(1, 10),
                    Operation::AdjacentSum(4, 14),
                    Operation::DiagonalSum(8, 10),
                    Operation::AdjacentSum(6, 8),
                    Operation::DiagonalSum(3, 5),
                    Operation::DiagonalSum(2, 5),
                    Operation::DiagonalSum(6, 9),
                    Operation::DiagonalSum(5, 12),
                    Operation::DiagonalSum(7, 5),
                    Operation::DiagonalSum(4, 9),
                    Operation::DiagonalSum(0, 5),
                ],
            ),
        ];

        for (grid, operations) in test_cases {
            let neighbor_sum = N::new(grid.to_vec());

            for operation in operations {
                match *operation {
                    Operation::AdjacentSum(value, expected) => assert_eq!(neighbor_sum.adjacent_sum(value), expected),
                    Operation::DiagonalSum(value, expected) => assert_eq!(neighbor_sum.diagonal_sum(value), expected),
                }
            }
        }
    }
}
