pub mod binary_heap;

pub trait Solution {
    fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[10, 5, 0], [15, 2, 1], [25, 1, 1], [30, 4, 0]] as &[_], 6),
            (
                &[[7, 1_000_000_000, 1], [15, 3, 0], [5, 999_999_995, 0], [5, 1, 1]],
                999_999_984,
            ),
            (
                &[
                    [26, 7, 0],
                    [16, 1, 1],
                    [14, 20, 0],
                    [23, 15, 1],
                    [24, 26, 0],
                    [19, 4, 1],
                    [1, 1, 0],
                ],
                34,
            ),
            (
                &[
                    [23, 8, 0],
                    [28, 29, 1],
                    [11, 30, 1],
                    [30, 25, 0],
                    [26, 9, 0],
                    [3, 21, 0],
                    [28, 19, 1],
                    [19, 30, 0],
                    [20, 9, 1],
                    [17, 6, 0],
                ],
                102,
            ),
            (&[[7, 13, 1], [7, 13, 0]], 0),
        ];

        for (orders, expected) in test_cases {
            assert_eq!(
                S::get_number_of_backlog_orders(orders.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
