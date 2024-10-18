pub mod iterative;

pub trait Solution {
    fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 7], [2, 6], [3, 5], [4, 4], [5, 4], [6, 3], [7, 2], [8, 1]] as &[_],
                3,
            ),
            (&[[3, 4], [1, 2], [7, 8], [2, 3]], 1),
            (
                &[
                    [1, 1],
                    [2, 2],
                    [3, 3],
                    [4, 5],
                    [5, 7],
                    [6, 6],
                    [7, 5],
                    [8, 4],
                    [9, 4],
                    [10, 4],
                ],
                4,
            ),
        ];

        for (stock_prices, expected) in test_cases {
            assert_eq!(S::minimum_lines(stock_prices.iter().map(Vec::from).collect()), expected);
        }
    }
}
